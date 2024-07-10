use actix::{Handler, Message, ResponseFuture};
use sqlx::Executor;

use crate::error::Error;
use crate::models::credential::Credential;

use super::Repository;

pub struct MaybeFindOneCredentialByNameOrEmail(pub String);

impl Message for MaybeFindOneCredentialByNameOrEmail {
    type Result = Result<Option<Credential>, Error>;
}

impl Handler<MaybeFindOneCredentialByNameOrEmail> for Repository {
    type Result = ResponseFuture<Result<Option<Credential>, crate::error::Error>>;

    fn handle(
        &mut self,
        msg: MaybeFindOneCredentialByNameOrEmail,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        Box::pin(async {
            let conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self.pool.acquire().await?;

            let cred: Credential =
                sqlx::query("SELECT id, password FROM users WHERE username=$1 OR WHERE email=$1")
                    .bind(msg.0)
                    .fetch_optional(conn)
                    .await?;

            Ok(cred)
        })
    }
}
