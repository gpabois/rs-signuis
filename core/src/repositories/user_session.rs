use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    error::Error,
    models::session::{UserSession, UserSessionId},
};

use super::RepositoryOp;

/// Insére une nouvelle session utilisateur dans la base de données.
pub struct InsertUserSession {
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
}

impl RepositoryOp for InsertUserSession {
    type Return = UserSessionId;

    fn execute<'c, E>(
        self,
        executor: E,
    ) -> futures::prelude::future::LocalBoxFuture<'c, Result<Self::Return, Error>>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Postgres> + 'c,
    {
        Box::pin(async move {
            let (id,): (UserSessionId,) = sqlx::query_as(
                "INSERT INTO user_sessions (token, user_id, expires_at) VALUES ($1, $2, $3) RETURNING id",
            )
            .bind(self.token)
            .bind(self.user_id)
            .bind(self.expires_at)
            .fetch_one(executor)
            .await?;

            Ok(id)
        })
    }
}

/// Récupère une session utilisateur valide (ex: pas expirée) derrière le jeton.
pub struct MaybeFindOneValidUserSessionByToken(pub String);

impl RepositoryOp for MaybeFindOneValidUserSessionByToken {
    type Return = Option<UserSession>;

    fn execute<'c, E>(
        self,
        executor: E,
    ) -> futures::prelude::future::LocalBoxFuture<'c, Result<Self::Return, Error>>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Postgres> + 'c,
    {
        Box::pin(async move {
            let session: Option<UserSession> = sqlx::query_as(
                "SELECT id, token, expires_at, created_at, 
                    user_id, user.username as user_name, user.email as user_email,
                    user.avatar as user_avatar, user.role as user_role
            FROM user_sessions
            INNER JOIN users AS user ON user_sessions.user_id = user.id
            WHERE token = $1",
            )
            .bind(self.0)
            .fetch_optional(executor)
            .await?;

            Ok(session)
        })
    }
}
