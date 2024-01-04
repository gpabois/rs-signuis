use std::ops::Add;

use chrono::{Utc, Duration};
use futures::future::BoxFuture;

use crate::{Error, entities::{session::Session, log::LogFilter}};

use super::logger::traits::Logger;

pub enum View {
    Administration
}
// Action
pub enum Action {
    CanRegister,
    CanAuthenticate
}

pub mod traits {
    use futures::future::BoxFuture;

    use crate::entities::session::Session;
    use crate::Error;

    pub trait Authorization<'q>: Sized{
        // The actor behind the session can ?
        fn can<'a, 'b>(self, actor: &'a Session, action: super::Action) 
            -> BoxFuture<'b, Result<bool, Error>>
        where 'a: 'b, 'q: 'b;
    }
}

impl<'q> traits::Authorization<'q> for &'q mut super::ServiceTx<'_> {
    fn can<'a, 'b>(self, actor: &'a Session, action: Action) -> BoxFuture<'b, Result<bool, Error>> 
    where 'a: 'b, 'q: 'b
    {
        Box::pin(async move {
            let can = match action {
                Action::CanRegister => actor.is_anonynmous(),
                Action::CanAuthenticate => {
                    let log_filter = LogFilter::And(vec![
                        LogFilter::TypeEq("authentication::failed".into()),
                        LogFilter::AtGte(Utc::now().add(Duration::minutes(-15))),
                        LogFilter::ClientIpEq(actor.client.ip.clone())
                    ]);
                    
                    let nb_of_failed_attempts = self.count_log_by(log_filter).await?;
                    // Only works if the actor is not authenticated.
                    actor.is_anonynmous() 
                    && (nb_of_failed_attempts < 3)
                }
            };
    
            return Ok(can)
        })
    }
}