use sql_gis::sql_types::PgPoint;
use uuid::Uuid;

use super::RepositoryOp;
use crate::{error::Error, models::nuisance_report::NuisanceReportId};

/// Objet pour insérer un signalement de nuisance.
pub struct InsertNuisanceReport {
    pub type_id: Uuid,
    pub user_id: Option<Uuid>,
    pub location: PgPoint,
    pub intensity: i8,
}

impl RepositoryOp for InsertNuisanceReport {
    type Return = NuisanceReportId;

    fn execute<'c, E>(
        self,
        executor: E,
    ) -> futures::prelude::future::LocalBoxFuture<'c, Result<Self::Return, Error>>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Postgres> + 'c,
    {
        Box::pin(async move {
            let (id,): (NuisanceReportId,) = sqlx::query_as(INSERT_NUISANCE_QUERY)
                .bind(self.type_id)
                .bind(self.location)
                .bind(self.intensity)
                .bind(self.user_id)
                .fetch_one(executor)
                .await?;

            Ok(id)
        })
    }
}

const INSERT_NUISANCE_QUERY: &str = r#"
    INSERT INTO nuisance_reports (type_id, location, intensity, user_id) 
        VALUES ($1, $2, $3, $4) 
    RETURNING id
"#;
