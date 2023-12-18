mod database;
mod account;
mod authentication;

pub use database::*;
pub use authentication::*;
pub use account::*;
use futures::future::BoxFuture;
use sqlx::{Postgres, Pool, Transaction, Acquire};

use crate::{Error, model::{session::Session, credentials::Credentials}, repositories::Repository};

pub struct Service {
    repos: Repository
}