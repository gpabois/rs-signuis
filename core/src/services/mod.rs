pub mod account;
pub mod authentication;
pub mod authorization;
pub mod database;
pub mod logger;
pub mod reporting;

use actix::{Actor, Addr, Context};
use chrono::Duration;

use crate::{events::EventBus, repositories::Repository};

pub struct ServiceSettings {
    user_session_expiration_time: Duration,
}

#[derive(Clone)]
pub struct Service {
    params: ServiceSettings,
    repos: Addr<Repository>,
    events: Addr<EventBus>,
}

impl Actor for Service {
    type Context = Context<Self>;
}
