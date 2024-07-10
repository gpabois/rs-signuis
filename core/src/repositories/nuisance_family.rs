use super::Repository;
use crate::{error::Error, models::nuisance_family::NuisanceFamilyId};
use actix::{Handler, Message, ResponseFuture};
use sqlx::{Acquire, Executor};

pub struct InsertNuisanceFamily {
    pub label: String,
    pub description: String,
}

impl Message for InsertNuisanceFamily {
    type Result = Result<NuisanceFamilyId, Error>;
}

impl Handler<InsertNuisanceFamily> for Repository {
    type Result = ResponseFuture<<InsertNuisanceFamily as Message>::Result>;

    fn handle(&mut self, msg: InsertNuisanceFamily, ctx: &mut Self::Context) -> Self::Result {
        Box::pin(async move {
            let conn = self.pool.acquire().await?;

            let (id,): (NuisanceFamilyId,) = sqlx::query(
                "INSERT INTO nuisance_families (label, description) VALUES ($1, $2) RETURNING id",
            )
            .bind(msg.label)
            .bind(msg.description)
            .execute(conn)
            .await?;

            Ok(id)
        })
    }
}
