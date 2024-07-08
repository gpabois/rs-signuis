pub mod database;
pub mod account;
pub mod authentication;
pub mod authorization;
pub mod logger;
pub mod reporting;

use actix::{Actor, Addr, Context};

use crate::repositories::Repository;

#[derive(Clone)]
pub struct Service {
    pub repos: Addr<Repository>
}

impl Actor for Service {
    type Context = Context<Self>;
}
