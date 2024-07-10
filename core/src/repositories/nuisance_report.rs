use actix::{Handler, Message, ResponseFuture};
use sql_gis::sql_types::PgGeometry;
use sqlx::Acquire;
use uuid::Uuid;

use super::Repository;
use crate::{error::Error, models::nuisance_report::NuisanceReportId};

/// Objet pour insérer un signalement de nuisance.
pub struct InsertNuisanceReport {
    pub type_id: Uuid,
    pub user_id: Option<Uuid>,
    pub location: PgGeometry,
    pub intensity: i8,
}

impl Message for InsertNuisanceReport {
    type Result = Result<NuisanceReportId, Error>;
}

impl Handler<InsertNuisanceReport> for Repository {
    type Result = ResponseFuture<Result<NuisanceReportId, Error>>;

    fn handle(&mut self, msg: InsertNuisanceReport, ctx: &mut Self::Context) -> Self::Result {
        Box::pin(async {
            let conn = self.pool.acquire().await?;

            sqlx::query_as("INSERT INTO nuisance_reports (type_id, location, intensity, user_id) VALUES ($1, $2, $3, $4) RETURNING id")
                .bind(msg.type_id)
                .bind(msg.location)
                .bind(msg.intensity)
                .bind(msg.user_id)
                .fetch_one(conn)
                .await
        })
    }
}
