use sql_builder::{bind, columns, id, insert, prelude::*, row_value, select, select_columns};
use uuid::Uuid;

use crate::error::Error;
use crate::models::nuisance_type::NuisanceTypeId;

use super::RepositoryOp;

const INSERT_NUISANCE_TYPE_QUERY: &str = r#"
    INSERT INTO nuisance_types (label, description, family_id) 
        VALUES ($1, $2, $3) 
    RETURNING id
"#;

const NUISANCE_TYPE_EXISTS_QUERY: &str = r#"
    SELECT EXISTS(
        SELECT id FROM nuisance_types WHERE id=$1
    )
"#;

const TABLE: sql_builder::identifier::IdentifierRef<'static> = id!(nuisance_types);

pub struct InsertNuisanceType {
    pub label: String,
    pub description: String,
    pub family_id: Uuid,
}

impl RepositoryOp for InsertNuisanceType {
    type Return = NuisanceTypeId;

    fn execute<'c, E>(
        self,
        executor: E,
    ) -> futures::prelude::future::LocalBoxFuture<'c, Result<Self::Return, Error>>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Postgres> + 'c,
    {
        Box::pin(async move {
            let (sql, args) = insert(TABLE)
                .columns(columns!(id!(label), id!(description), id!(family_id)))
                .values(row_value!(
                    bind!(&self.label),
                    bind!(&self.description),
                    bind!(&self.family_id)
                ))
                .build::<sqlx::Postgres>();

            let (id,): (NuisanceTypeId,) =
                sqlx::query_as_with(&sql, args).fetch_one(executor).await?;

            Ok(id)
        })
    }
}

pub struct NuisanceTypeExists(pub NuisanceTypeId);

impl RepositoryOp for NuisanceTypeExists {
    type Return = bool;

    fn execute<'c, E>(
        self,
        executor: E,
    ) -> futures::prelude::future::LocalBoxFuture<'c, Result<Self::Return, Error>>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Postgres> + 'c,
    {
        Box::pin(async move {
            let (exists,): (bool,) = sqlx::query_as(NUISANCE_TYPE_EXISTS_QUERY)
                .bind(self.0)
                .fetch_one(executor)
                .await?;
            Ok(exists)
        })
    }
}
