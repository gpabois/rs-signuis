pub mod database;
pub mod account;
pub mod authentication;
pub mod authorization;
pub mod logger;
pub mod reporting;

use sqlx::PgPool;

use crate::repositories::Repository;

#[derive(Clone)]
pub struct Service{
    pub pool: PgPool,
    pub repos: Repository
}
