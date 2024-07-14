use actix::{Actor, Addr, Context, Handler, Message, Recipient};

macro_rules! impl_event {
    ($event: ident) => {
        paste::paste! {
            impl actix::Message for $event {
                type Result = ();
            }

            impl actix::Handler<$event> for super::EventBusActor {
                type Result = ();

                fn handle(&mut self, msg: $event, _ctx: &mut Self::Context) -> Self::Result {
                    self.[<$event:snake _subscribers>]
                        .iter()
                        .for_each(|sub| sub.do_send(msg.clone()))
                }
            }

            #[derive(actix::prelude::Message)]
            #[rtype(result = "()")]
            pub struct [<On $event>](actix::Recipient<$event>);

            impl actix::Handler<[<On $event>]> for super::EventBusActor {
                type Result = ();

                fn handle(&mut self, msg: [<On $event>], _ctx: &mut Self::Context) -> Self::Result {
                    self.[<$event:snake _subscribers>].push(msg.0)
                }
            }
        }
    };
}

mod authentication_failed;
mod user_registered;

pub use authentication_failed::*;
pub use user_registered::*;

#[derive(Default)]
/// Bus évènementiel
pub struct EventBusActor {
    pub user_registered_subscribers: Vec<Recipient<UserRegistered>>,
    pub authentication_failed_subscribers: Vec<Recipient<AuthenticationFailed>>,
}

impl Actor for EventBusActor {
    type Context = Context<Self>;
}

#[derive(Clone)]
pub struct EventBus(Addr<EventBusActor>);

impl EventBus {
    pub fn new() -> Self {
        Self(EventBusActor::default().start())
    }

    pub fn notify<E>(&self, event: E)
    where
        EventBusActor: Handler<E>,
        E: Message + Sync + Send + 'static,
        E::Result: Sync + Send,
    {
        self.0.do_send(event)
    }
}
