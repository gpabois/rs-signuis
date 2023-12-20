use futures::future::BoxFuture;
use sqlx::Acquire;

use crate::{model::user::{RegisterUser, User, UserFilter, InsertUser}, Error, Issue, Issues, repositories::users::traits::UserRepository};
use crate::services::authorization::traits::Authorization;

use super::authorization::Action;

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{model::{user::{User, RegisterUser}, session::Session}, Error};

    pub trait Account<'q> {
        /// Register a new user
        fn register_user<'a, 'b>(self, args: RegisterUser, actor: &'a Session) -> BoxFuture<'b, Result<User, Error>> where 'q: 'b, 'a: 'q;
    }
}

impl<'q> traits::Account<'q> for &'q mut super::ServiceTx<'_> {
    fn register_user<'a, 'b>(self, args: RegisterUser, actor: &'a crate::model::session::Session) -> BoxFuture<'b, Result<User, Error>> where 'q: 'b, 'a: 'q {
        Box::pin(async {
            //
            self.can(actor, Action::CanRegister).await?;            
            
            let querier = self.querier.acquire().await?;
            
            let mut issues = Issues::new();
            issues.async_add_if_not_true(self.repos.any_users_by(&mut *querier, UserFilter::name(&args.name)), Issue::new(
                "form".into(),
                "Le nom d'utilisateur est déjà pris.".into(),
                vec!["name".into()]
            )).await?;

            issues.async_add_if_not_true(self.repos.any_users_by(&mut *querier, UserFilter::email(&args.email)), Issue::new(
                "form".into(),
                "L'email est déjà pris.".into(),
                vec!["email".into()]
            )).await?;

            
            issues.validate()?;

            let insert: InsertUser = args.into();
            self.repos.insert_user(querier, insert).await
        })
    }
}
