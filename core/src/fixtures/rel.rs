use futures::future::BoxFuture;
use crate::{services::ServiceTx, entities::Identifiable, Error};

use super::Fixture;

#[derive(Clone)]
pub enum ForeignKeyFixture<ID, F> {
    ID(ID),
    Fixture(F)
}

impl<ID, F> Default for ForeignKeyFixture<ID, F> where F: Default {
    fn default() -> Self {
        Self::Fixture(F::default())
    }
}

impl<F> From<&'_ str> for ForeignKeyFixture<String, F> {
    fn from(value: &'_ str) -> Self {
        Self::ID(value.into())
    }
}

impl<ID, E, F> From<&E> for ForeignKeyFixture<ID, F> where F: Fixture<Entity=E>, E: Identifiable<Type=ID>  {
    fn from(value: &E) -> Self {
        Self::ID(value.id())
    }
}

impl<ID, F> ForeignKeyFixture<ID, F> {
    pub fn expect_id(self, msg: &str) -> ID {
        match self {
            Self::ID(id) => id,
            _ => unreachable!("{}", msg)
        }
    }
}

impl<ID, F> Fixture for ForeignKeyFixture<ID, F> 
    where F: super::Fixture + std::marker::Send + 'static, F::Entity: Identifiable<Type=ID>, ID: std::marker::Send + 'static
{
    type Entity = Self;

    fn into_entity<'a, 'b>(self, tx: &'a mut ServiceTx<'_>) -> BoxFuture<'b, Result<Self::Entity, Error>> where 'a: 'b {
        Box::pin(async move {
            Ok(match self {
                Self::Fixture(args) => Self::ID(args.into_entity(tx).await?.id()),
                Self::ID(id) => Self::ID(id)
            })
        })
    }
}