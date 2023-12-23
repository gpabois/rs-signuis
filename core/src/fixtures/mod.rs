use futures::future::BoxFuture;
use crate::{services::ServiceTx, Error};

pub mod clients;
pub mod users;
pub mod sessions;
pub mod rel;

pub trait Fixture {
    type Entity;

    fn into_entity<'a, 'b>(self, tx: &'a mut ServiceTx<'_>) -> BoxFuture<'b, Result<Self::Entity, Error>> where 'a: 'b;
}