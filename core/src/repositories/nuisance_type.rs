use actix::{Handler, Message, ResponseFuture};
use sqlx::Acquire;
use uuid::Uuid;

use crate::error::Error;
use crate::models::nuisance_type::NuisanceTypeId;

use super::Repository;

pub struct InsertNuisanceType {
    pub label: String,
    pub description: String,
    pub family_id: Uuid,
}

impl Message for InsertNuisanceType {
    type Result = Result<NuisanceTypeId, Error>;
}

impl Handler<InsertNuisanceType> for Repository {
    type Result = ResponseFuture<Result<NuisanceTypeId, Error>>;

    fn handle(&mut self, msg: InsertNuisanceType, ctx: &mut Self::Context) -> Self::Result {
        Box::pin(async {
            let conn = self.pool.acquire().await?;

            sqlx::query_as("INSERT INTO nuisance_types (label, description, family_id) VALUES ($1, $2, $3) RETURNING id")
                .bind(msg.label)
                .bind(msg.description)
                .bind(msg.family_id)
                .fetch_one(conn)
                .await
        })
    }
}
