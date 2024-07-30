use sql_builder::{bind, columns, id, insert, prelude::*, row_value};
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
            let (sql, args) = insert(TABLE)
                .columns(columns!(
                    id!(type_id),
                    id!(location),
                    id!(intensity),
                    id!(user_id)
                ))
                .values(row_value!(
                    bind!(self.type_id),
                    bind!(self.location),
                    bind!(self.intensity),
                    bind!(self.user_id)
                ))
                .build::<::sqlx::Postgres>();

            let (id,) = sqlx::query_as_with(&sql, args).fetch_one(executor).await?;

            Ok(id)
        })
    }
}

const TABLE: sql_builder::identifier::IdentifierRef<'static> = id!(nuisance_reports);
