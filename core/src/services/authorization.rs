use crate::{Error, model::session::Session};


pub enum View {
    Administration
}
// Action
pub enum Action {
    CanRegister,
}

pub mod traits {
    use futures::future::BoxFuture;

    use crate::model::session::Session;
    use crate::Error;

    pub trait Authorization<'q> {
        // The actor behind the session can ?
        fn can<'a>(self, actor: &Session, action: super::Action) -> BoxFuture<'a, Result<(), Error>>;
    }
}

impl<'q> traits::Authorization<'q> for &'q mut super::ServiceTx<'_> {
    fn can<'a>(self, actor: &Session, action: Action) -> futures::prelude::future::BoxFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            let can = match action {
                Action::CanRegister => actor.is_anonynmous()
            };
    
            if !can {
                return Err(Error::unauthorized());
            }
    
            Ok(())
        })
    }
}