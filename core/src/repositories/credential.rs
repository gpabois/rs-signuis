use sql_builder::helpers::QuerySpecification;
use sql_builder::{bind, eq, id, or, prelude::*, select, select_columns};

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
            let (sql, args) = select(select_columns!(id!(id), id!(password)))
                .from(TABLE)
                .r#where(or(
                    eq(id!(username), bind!(&self.0)),
                    eq(id!(email), bind!(&self.0)),
                ))
                .build::<sqlx::Postgres>();

            let credential: Option<Credential> = sqlx::query_as_with(&sql, args)
                .fetch_optional(executor)
                .await?;

            Ok(credential)
        })
    }
}

const TABLE: sql_builder::identifier::IdentifierRef<'static> = id!(users);
