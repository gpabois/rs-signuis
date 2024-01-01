use async_stream::stream;
use fake::{faker::lorem::en::{Word, Paragraph}, Fake};
use futures::{future::BoxFuture, stream::BoxStream};
use sqlx::Acquire;
use uuid::Uuid;

use crate::{model::report::{NuisanceFamily, InsertNuisanceFamily, NewNuisanceFamily}, Error, services::ServiceTx, repositories::nuisance_families::traits::NuisanceFamilyRepository};

use super::Fixture;

#[derive(Default, Clone)]
pub struct NuisanceFamilyFixture {
    id: Option<Uuid>,
    label: Option<String>,
    description: Option<String>
}

impl NuisanceFamilyFixture {
    pub fn new() -> Self {
        Self::default()
    }
}

impl NuisanceFamilyFixture {
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
}

impl Into<NewNuisanceFamily> for NuisanceFamilyFixture {
    fn into(self) -> NewNuisanceFamily {
        NewNuisanceFamily {
            label: self.label.unwrap_or_else(|| Word().fake()),
            description: self.description.unwrap_or_else(|| Paragraph(30..120).fake()),
        }
    }   
}

impl Into<InsertNuisanceFamily> for NuisanceFamilyFixture {
    fn into(self) -> InsertNuisanceFamily {
        InsertNuisanceFamily {
            id: self.id,
            label: self.label.unwrap_or_else(|| Paragraph(2..4).fake()),
            description: self.description.unwrap_or_else(|| Paragraph(30..120).fake()),
        }
    }
}

impl super::Fixture for NuisanceFamilyFixture {
    type Entity = NuisanceFamily;

    fn into_entity<'a, 'b>(self, tx: &'a mut ServiceTx<'_>) -> BoxFuture<'b, Result<Self::Entity, Error>> where 'a: 'b {
        Box::pin(async move {
            let querier: &mut sqlx::PgConnection = tx.querier.acquire().await?;
            tx.repos.insert_nuisance_family(querier, self.into()).await
        })
    }
}

pub fn new_multi<'a, 'b, 'q>(tx: &'a mut ServiceTx<'q>) -> BoxStream<'b, Result<NuisanceFamily, Error>> 
where 'a: 'b
{
    Box::pin(stream! {loop {
        yield NuisanceFamilyFixture::new().into_entity(tx).await
    }})
}
