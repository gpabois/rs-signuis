use super::RepositoryOp;
use crate::{
    error::Error,
    models::nuisance_family::{NuisanceFamily, NuisanceFamilyId},
};

use sql_builder::{bind, columns, id, insert, prelude::*, row_value, select, select_columns};

const TABLE: sql_builder::identifier::IdentifierRef<'static> = id!(nuisance_families);

pub struct InsertNuisanceFamily {
    pub label: String,
    pub description: String,
}

impl RepositoryOp for InsertNuisanceFamily {
    type Return = NuisanceFamilyId;

    fn execute<'c, E>(
        self,
        executor: E,
    ) -> futures::prelude::future::LocalBoxFuture<'c, Result<Self::Return, Error>>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Postgres> + 'c,
    {
        Box::pin(async move {
            let (sql, args) = insert(TABLE)
                .columns(columns!(id!(label), id!(description)))
                .values(row_value!(bind!(&self.label), bind!(&self.description)))
                .build::<::sqlx::Postgres>();

            ::sqlx::query_as_with(&sql, args)
                .fetch_one(executor)
                .await?;

            Ok(id)
        })
    }
}
pub struct NuisanceFamilyExists(pub NuisanceFamilyId);

impl RepositoryOp for NuisanceFamilyExists {
    type Return = bool;

    fn execute<'c, E>(
        self,
        executor: E,
    ) -> futures::prelude::future::LocalBoxFuture<'c, Result<Self::Return, Error>>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Postgres> + 'c,
    {
        Box::pin(async move {
            let (exists,): (bool,) =
                sqlx::query_as("SELECT EXISTS(SELECT id FROM nuisance_families WHERE id = $1)")
                    .bind(self.0)
                    .fetch_one(executor)
                    .await?;

            Ok(exists)
        })
    }
}

/// Récupère des familles de nuisance depuis le répertoire.
pub struct FetchNuisanceFamilies {}

impl FetchNuisanceFamilies {
    pub fn all() -> Self {
        Self {}
    }
}

impl RepositoryOp for FetchNuisanceFamilies {
    type Return = Vec<NuisanceFamily>;

    fn execute<'c, E>(
        self,
        executor: E,
    ) -> futures::prelude::future::LocalBoxFuture<'c, Result<Self::Return, Error>>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Postgres> + 'c,
    {
        Box::pin(async move {
            let (sql, _) = select(select_columns!(id!(id), id!(label), id!(description)))
                .from(TABLE)
                .build::<sqlx::Postgres>();

            let nuisance_families =
                sqlx::query_as("SELECT id, label, description FROM nuisance_families")
                    .fetch_all(executor)
                    .await?;

            Ok(nuisance_families)
        })
    }
}
