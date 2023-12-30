use chrono::{Utc, DateTime};
use futures::future::BoxFuture;
use geojson::Geometry;
use rand::Rng;
use sqlx::Acquire;
use uuid::Uuid;

use crate::{Error, model::report::{NewNuisanceReport, InsertNuisanceReport, NuisanceReport}, services::ServiceTx, repositories::nuisance_reports::traits::NuisanceReportRepository};

use super::{rel::ForeignKeyFixture, Fixture, users::UserFixture, nuisance_types::NuisanceTypeFixture};

#[derive(Default, Clone)]
pub struct NuisanceReportFixture {
    id: Option<Uuid>,
    user_id: Option<ForeignKeyFixture<Uuid, UserFixture>>,
    type_id: Option<ForeignKeyFixture<Uuid, NuisanceTypeFixture>>,
    location: Option<Geometry>,
    intensity: Option<i8>,
    created_at: Option<DateTime<Utc>>
}

impl NuisanceReportFixture {
    pub fn new() -> Self {
        Self::default()
    }
}

impl NuisanceReportFixture {
    pub fn with_id<I: Into<Uuid>>(&mut self, value: I) -> &mut Self {
        self.id = Some(value.into());
        self
    }

    pub fn with_intensity<I: Into<i8>>(&mut self, value: I) -> &mut Self {
        self.intensity = Some(value.into());
        self
    }
    pub fn with_location<G: Into<Geometry>>(&mut self, value: G) -> &mut Self {
        self.location = Some(value.into());
        self
    }

    pub fn with_user<FK: Into<ForeignKeyFixture<Uuid, UserFixture>>>(&mut self, value: FK) -> &mut Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn with_type<FK: Into<ForeignKeyFixture<Uuid, NuisanceTypeFixture>>>(&mut self, value: FK) -> &mut Self {
        self.type_id = Some(value.into());
        self
    }

    pub async fn create_deps<'a>(&mut self, tx: &'a mut ServiceTx<'_>) -> Result<&mut Self, Error> {
        self.type_id = Some(
            self.type_id
            .clone()
            .unwrap_or_else(|| Default::default())
            .into_entity(tx)
            .await?
        );

        if let Some(fk) = self.user_id.clone() {
            self.user_id = Some(
                fk.into_entity(tx).await?
            )
        }

        Ok(self)
    }
}



impl Into<NewNuisanceReport> for NuisanceReportFixture {
    fn into(self) -> NewNuisanceReport {
        NewNuisanceReport {
            location: self.location.unwrap_or_else(super::geojson::random_point),
            intensity: rand::thread_rng().gen_range(1..5),
            user_id: self.user_id
            .map(|id| 
                id.expect_id("cannot generate nuisance report insertion data with pending user fixture")
            ),
            type_id: self
                .type_id
                .unwrap()
                .expect_id("cannot generate nuisance report insertion data with pending type fixture")
        }
    }  
}

impl Into<InsertNuisanceReport> for NuisanceReportFixture {
    fn into(self) -> InsertNuisanceReport {
        InsertNuisanceReport {
            id: self.id,
            location: self.location.unwrap_or_else(super::geojson::random_point),
            intensity: rand::thread_rng().gen_range(1..5),
            user_id: self.user_id
            .map(|id| 
                id.expect_id("cannot generate nuisance report insertion data with pending user fixture")
            ),
            type_id: self
                .type_id
                .unwrap()
                .expect_id("cannot generate nuisance report insertion data with pending type fixture"),
            created_at: self.created_at
        }
    }
}

impl super::Fixture for NuisanceReportFixture {
    type Entity = NuisanceReport;

    fn into_entity<'a, 'b>(mut self, tx: &'a mut ServiceTx<'_>) -> BoxFuture<'b, Result<Self::Entity, Error>> where 'a: 'b {
        Box::pin(async move {
            self.create_deps(tx).await?;
            let querier: &mut sqlx::PgConnection = tx.querier.acquire().await?;
            tx.repos.insert_nuisance_report(querier, self.into()).await
        })
    }
}