use actix::{Actor, Context, Recipient};

mod user_registered;

pub use user_registered::*;

/// Bus évènementiel
pub struct EventBus {
    pub(self) user_registered_subscribers: Vec<Recipient<UserRegistered>>,
}

impl Actor for EventBus {
    type Context = Context<Self>;
}
