use actix::{Handler, ResponseFuture};
use sqlx::Executor;

use crate::models::credential::Credential;

use super::Repository;

pub struct FindOneCredentialByNameOrEmail(pub String);

impl Handler<FindOneCredentialByNameOrEmail> for Repository {
    type Result = ResponseFuture<Result<Option<Credential>, crate::error::Error>>;

    fn handle(&mut self, msg: FindOneCredentialByNameOrEmail, ctx: &mut Self::Context) -> Self::Result {
        Box::pin(async {
            let conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self.pool.acquire().await?;

            let cred: Credential = sqlx::query("SELECT id, password FROM users WHERE username=$1 OR WHERE email=$1")
                .bind(msg.0)
                .fetch_optional(conn)
                .await?;
    
            Ok(cred)
        })
    }
}

