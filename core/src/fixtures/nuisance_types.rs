use async_stream::stream;
use fake::{faker::lorem::en::{Paragraph, Word}, Fake};
use futures::{future::BoxFuture, stream::BoxStream};
use sqlx::Acquire;
use crate::types::uuid::Uuid;

use crate::{Error, entities::nuisance::{InsertNuisanceType, NuisanceType, CreateNuisanceType}, services::ServiceTx, repositories::nuisance_types::traits::NuisanceTypeRepository};

use super::{rel::ForeignKeyFixture, nuisance_families::NuisanceFamilyFixture, Fixture};

#[derive(Default, Clone)]
pub struct NuisanceTypeFixture {
    id: Option<Uuid>,
    label: Option<String>,
    description: Option<String>,
    family_id: Option<ForeignKeyFixture<Uuid, NuisanceFamilyFixture>>
}

impl NuisanceTypeFixture {
    pub fn new() -> Self {
        Self::default()
    }
}

impl NuisanceTypeFixture {
    pub fn with_id<I: Into<Uuid>>(&mut self, value: I) -> &mut Self {
        self.id = Some(value.into());
        self
    }

    pub fn with_label<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.label = Some(value.into());
        self
    }
    pub fn with_description<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.description = Some(value.into());
        self
    }

    pub fn with_family<FK: Into<ForeignKeyFixture<Uuid, NuisanceFamilyFixture>>>(&mut self, value: FK) -> &mut Self {
        self.family_id = Some(value.into());
        self
    }

    pub async fn create_deps<'a>(&mut self, tx: &'a mut ServiceTx<'_>) -> Result<&mut Self, Error> {
        self.family_id = Some(
            self.family_id
            .clone()
            .unwrap_or_else(|| Default::default())
            .into_entity(tx)
            .await?
        );

        Ok(self)
    }
}

impl Into<CreateNuisanceType> for NuisanceTypeFixture {
    fn into(self) -> CreateNuisanceType {
        CreateNuisanceType {
            label: self.label.unwrap_or_else(|| Word().fake()),
            description: self.description.unwrap_or_else(|| Paragraph(30..120).fake()),
            family_id: self
                .family_id
                .unwrap()
                .expect_id("cannot generate nuisance type insertion data with pending family fixture")
        }
    }  
}

impl Into<InsertNuisanceType> for NuisanceTypeFixture {
    fn into(self) -> InsertNuisanceType {
        InsertNuisanceType {
            id: self.id,
            label: self.label.unwrap_or_else(|| Paragraph(2..4).fake_with_rng(&mut rand::thread_rng())),
            description: self.description.unwrap_or_else(|| Paragraph(30..120).fake_with_rng(&mut rand::thread_rng())),
            family_id: self
                .family_id
                .unwrap()
                .expect_id("cannot generate nuisance type insertion data with pending family fixture")
        }
    }
}

impl super::Fixture for NuisanceTypeFixture {
    type Entity = NuisanceType;

    fn into_entity<'a, 'b>(mut self, tx: &'a mut ServiceTx<'_>) -> BoxFuture<'b, Result<Self::Entity, Error>> where 'a: 'b {
        Box::pin(async move {
            self.create_deps(tx).await?;
            let querier: &mut sqlx::PgConnection = tx.querier.acquire().await?;
            tx.repos.insert_nuisance_type(querier, self).await
        })
    }
}

pub fn new_multi_with<'a, 'b, 'q, F: Fn() -> NuisanceTypeFixture + Send + 'b>(tx: &'a mut ServiceTx<'q>, f: F) -> BoxStream<'b, Result<NuisanceType, Error>> 
where 'a: 'b
{
    Box::pin(stream! {
        loop {
            yield f().into_entity(tx).await
        }
    })
}