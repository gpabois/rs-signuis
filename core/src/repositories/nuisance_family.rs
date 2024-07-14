use super::RepositoryOp;
use crate::{
    error::Error,
    models::nuisance_family::{NuisanceFamily, NuisanceFamilyId},
};

const INSERT_NUISANCE_FAMILY_SQL: &str =
    "INSERT INTO nuisance_families (label, description) VALUES ($1, $2) RETURNING id";

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
            let (id,): (NuisanceFamilyId,) = sqlx::query_as(INSERT_NUISANCE_FAMILY_SQL)
                .bind(self.label)
                .bind(self.description)
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
            let nuisance_families =
                sqlx::query_as("SELECT id, label, description FROM nuisance_families")
                    .fetch_all(executor)
                    .await?;

            Ok(nuisance_families)
        })
    }
}
