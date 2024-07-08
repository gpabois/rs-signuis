use sqlx::Executor;
use super::Repository;
use crate::{error::Error, models::nuisance_family::{InsertNuisanceFamily, NuisanceFamilyId}};

impl Repository {
    async fn insert_nuisance_family<'c, I: Into<InsertNuisanceFamily>, E: Executor<'c>>(&mut self, executor: E, insert: I) -> Result<NuisanceFamilyId, Error> {
        let insert = InsertNuisanceFamily::from(insert);
        
        let (id,): (NuisanceFamilyId,) = sqlx::query("INSERT INTO nuisance_families (label, description) VALUES ($1, $2) RETURNING id")
        .bind(insert.label)
        .bind(insert.description)
        .execute(executor)
        .await?;

        Ok(id)
    }
}
