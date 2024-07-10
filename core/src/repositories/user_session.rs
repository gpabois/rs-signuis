use actix::{Handler, Message, ResponseFuture};
use chrono::{DateTime, Utc};
use sqlx::Acquire;
use uuid::Uuid;

use crate::{error::Error, models::user_session::UserSessionId};

use super::Repository;

/// Objet pour insérer une nouvelle session utilisateur.
pub struct InsertUserSession {
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
}

impl Message for InsertUserSession {
    type Result = Result<UserSessionId, Error>;
}

impl Handler<InsertUserSession> for Repository {
    type Result = ResponseFuture<Result<UserSessionId, Error>>;

    fn handle(&mut self, msg: InsertUserSession, ctx: &mut Self::Context) -> Self::Result {
        Box::pin(async {
            let conn = self.pool.acquire().await?;

            sqlx::query_as(
                "INSERT INTO user_sessions (token, user_id, expires_at) VALUES ($1, $2, $3) RETURNING id",
            )
            .bind(msg.token)
            .bind(msg.user_id)
            .bind(msg.expires_at)
            .await
        })
    }
}

#[derive(sqlx::FromRow)]
/// Session utilisateur
pub struct UserSession {
    pub id: Uuid,
    #[sqlx(flatten)]
    pub user: SessionUser,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
pub struct SessionUser {
    #[sqlx(rename = "user_id")]
    pub id: Uuid,

    #[sqlx(rename = "user_username")]
    pub username: String,

    #[sqlx(rename = "user_email")]
    pub email: String,

    #[sqlx(rename = "user_avatar")]
    pub avatar: Option<String>,
}

/// Récupère une session utilisateur valide (ex: pas expirée) derrière le jeton.
pub struct MaybeFindOneValidUserSessionByToken(pub String);

impl Message for MaybeFindOneValidUserSessionByToken {
    type Result = Result<Option<UserSession>, Error>;
}

impl Handler<MaybeFindOneValidUserSessionByToken> for Repository {
    type Result = ResponseFuture<Result<Option<UserSession>, Error>>;

    fn handle(
        &mut self,
        msg: MaybeFindOneValidUserSessionByToken,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        Box::pin(async {
            let conn = self.pool.acquire().await?;
            sqlx::query_as(
                "SELECT id, token, expires_at, created_at, 
                    user_id, user.username as user_name, user.email as user_email,
                    user.avatar as user_avatar
            FROM user_sessions
            INNER JOIN users AS user ON user_sessions.user_id = user.id
            WHERE token = $1",
            )
            .bind(msg.0)
            .fetch_optional(conn)
            .await
        })
    }
}
