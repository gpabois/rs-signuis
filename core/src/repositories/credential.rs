use crate::error::Error;
use crate::models::credential::Credential;

use super::RepositoryOp;

pub struct MaybeFindOneCredentialByNameOrEmail(pub String);

impl RepositoryOp for MaybeFindOneCredentialByNameOrEmail {
    type Return = Option<Credential>;

    fn execute<'c, E>(
        self,
        executor: E,
    ) -> futures::prelude::future::LocalBoxFuture<'c, Result<Self::Return, Error>>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Postgres> + 'c,
    {
        Box::pin(async move {
            let credential: Option<Credential> = sqlx::query_as(
                r#"
                SELECT id, password 
                FROM users 
                WHERE username=$1 OR WHERE email=$1
            "#,
            )
            .bind(self.0)
            .fetch_optional(executor)
            .await?;

            Ok(credential)
        })
    }
}
