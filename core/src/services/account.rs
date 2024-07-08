use futures::future::BoxFuture;
use sqlx::Acquire;

use crate::{models::user::{InsertUser, RegisterUser, User, UserFilter}, repositories::users::traits::UserRepository, validation::{Validation, Validator}, Error, Issue, Issues, Validator};
use crate::services::authorization::traits::Authorization;

use super::{authorization::Action, Service};

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{models::{user::{User, RegisterUser}, user_session::Session}, Error};

    pub trait Account<'q> {
        /// Register a new user
        fn register_user<'a, 'b>(self, args: RegisterUser, actor: &'a Session) -> BoxFuture<'b, Result<User, Error>> where 'q: 'b, 'a: 'q;
    }
}

impl Validation for RegisterUser {   
    fn assert(&self, validator: &mut Validator) {
        validator.assert_valid_email(&self.email, Some("l'adresse courriel est invalide"), ["email"]);
        validator.assert_eq(&self.password, &self.confirm_password, Some("les mots de passe ne sont pas égaux"), ["confirm_password"]);
        validator.assert_not_empty(&self.password, Some("le mot de passe ne doit pas être vide"), ["password"]);
        validator.assert_not_empty(&self.name, Some("le nom d'utilisateur ne doit pas être vide"), ["username"]);

    }
}

impl Service 
{
    /// Enregistre un nouvel utilisateur.
    async fn register_user(&mut self, register: RegisterUser) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        let mut validator = Validator::default();
        register.assert(&mut validator);
        validator.check()?;

        let (name_exists, email_exists) = self
            .repos
            .user_with_email_or_name_exists(&mut tx, &register.name, &register.email)
            .await?;
        
        validator.assert_not_true(name_exists , Some("le nom d'utilisateur est déjà pris"), ["username"]);
        validator.assert_not_true(email_exists , Some("l'adresse courriel est déjà pris"), ["email"]);

        self.repos.insert_user(&mut tx, register).await?;

        tx.commit();
    }   
}