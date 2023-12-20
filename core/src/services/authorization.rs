use futures::future::BoxFuture;

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
        fn can<'a: 'q+'b, 'b: 'q>(self, actor: &'a Session, action: super::Action) -> BoxFuture<'b, Result<(), Error>>;
    }
}

impl<'q> traits::Authorization<'q> for &'q mut super::ServiceTx<'_> {
    fn can<'a: 'q+'b, 'b: 'q>(self, actor: &'a Session, action: Action) -> BoxFuture<'b, Result<(), Error>> {
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