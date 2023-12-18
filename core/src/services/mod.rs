mod database;
mod account;
mod authentication;

use crate::repositories::Repository;

pub struct Service {
    repos: Repository
}