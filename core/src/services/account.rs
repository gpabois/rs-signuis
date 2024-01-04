use futures::future::BoxFuture;
use sqlx::Acquire;

use crate::{entities::user::{RegisterUser, User, UserFilter, InsertUser}, Error, Issue, Issues, repositories::users::traits::UserRepository, Validator};
use crate::services::authorization::traits::Authorization;

use super::authorization::Action;

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{entities::{user::{User, RegisterUser}, session::Session}, Error};

    pub trait Account<'q> {
        /// Register a new user
        fn register_user<'a, 'b>(self, args: RegisterUser, actor: &'a Session) -> BoxFuture<'b, Result<User, Error>> where 'q: 'b, 'a: 'q;
    }
}

impl Validator for &'_ RegisterUser {
    fn validate(self, issues: &mut Issues) {
        issues.email(&self.email, 
            Issue::new(
                "invalid_form".into(), 
                "Email invalide".into(), 
                vec!["email".into()]
            )
        );

        issues.eq(&self.password, &self.confirm_password, Issue::new(
            "invalid_form".into(), 
            "Les mots de passe ne sont pas égaux".into(), 
            vec!["confirm_password".into()]
        ));

        issues.not_empty(&self.password, Issue::new(
            "invalid_form".into(), 
            "Le mot de passe ne doit pas être vide".into(), 
            vec!["password".into()]
        ));

        issues.not_empty(&self.name, Issue::new(
            "invalid_form".into(), 
            "Le nom d'utilisateur ne doit pas être vide".into(), 
            vec!["name".into()]
        ));


    }
}

impl<'q> traits::Account<'q> for &'q mut super::ServiceTx<'_> {
    fn register_user<'a, 'b>(self, args: RegisterUser, actor: &'a crate::entities::session::Session) -> BoxFuture<'b, Result<User, Error>> where 'q: 'b, 'a: 'q {
        Box::pin(async {
            // Check if the actor can register a user.
            self.can(actor, Action::CanRegister).await?;            
        
            // Validate the arguments
            let mut issues = Issues::new();
            args.validate(&mut issues);
            issues.assert_valid()?; // Avoid a round to the database.

            // Additional checks such as exiting name and email.
            let querier = self.querier.acquire().await?;
            issues.async_true(self.repos.any_users_by(&mut *querier, UserFilter::name(&args.name)), Issue::new(
                "invalid_form".into(),
                "Le nom d'utilisateur est déjà pris.".into(),
                vec!["name".into()]
            )).await?;

            issues.async_true(self.repos.any_users_by(&mut *querier, UserFilter::email(&args.email)), Issue::new(
                "invalid_form".into(),
                "L'email est déjà pris.".into(),
                vec!["email".into()]
            )).await?;
            issues.assert_valid()?;

            // We can insert a new user into the database.
            let insert: InsertUser = args.into();
            self.repos
            .insert_user(querier, insert)
            .await
        })
    }
}
