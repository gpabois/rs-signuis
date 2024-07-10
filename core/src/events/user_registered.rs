use actix::prelude::*;

use super::EventBus;
use crate::models::user::UserId;

#[derive(Message, Clone, Copy)]
#[rtype(result = "()")]
pub struct UserRegistered(pub UserId);

impl Handler<UserRegistered> for EventBus {
    type Result = ();

    fn handle(&mut self, msg: UserRegistered, ctx: &mut Self::Context) -> Self::Result {
        ctx.user_register_subscribers
            .iter()
            .for_each(|sub| sub.send(msg))
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct OnUserRegistered(Recipient<UserRegistered>);

impl Handler<OnUserRegistered> for EventBus {
    type Result = ();

    fn handle(&mut self, msg: OnUserRegistered, ctx: &mut Self::Context) -> Self::Result {
        ctx.user_register_subscribers.push(msg.0)
    }
}
