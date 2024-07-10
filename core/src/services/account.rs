use actix::{Handler, Message, ResponseFuture};

use crate::{
    error::Error,
    events::UserRegistered,
    forms::account::RegisterUserForm,
    models::user::UserId,
    repositories::user::{InsertUser, UserWithUsernameOrEmailExists},
    validation::{Validation, Validator},
};

use super::Service;

pub struct RegisterUser {
    form: RegisterUserForm,
}

impl Message for RegisterUser {
    type Result = Result<UserId, Error>;
}

impl Handler<RegisterUser> for Service {
    type Result = ResponseFuture<Result<UserId, Error>>;

    fn handle(&mut self, msg: RegisterUser, ctx: &mut Self::Context) -> Self::Result {
        Box::pin(async {
            let mut validator = Validator::default();
            msg.form.assert(&mut validator);
            validator.check()?;

            let exists = self
                .repos
                .send(UserWithUsernameOrEmailExists {
                    username: msg.form.username.clone(),
                    email: msg.form.email.clone(),
                })
                .await??;

            validator.assert_not_true(
                exists.username_exists,
                Some("le nom d'utilisateur est déjà pris"),
                ["username"],
            );

            validator.assert_not_true(
                exists.email_exists,
                Some("l'adresse courriel est déjà pris"),
                ["email"],
            );

            let user_id = self
                .repos
                .send(InsertUser {
                    username: msg.form.username,
                    email: msg.form.email,
                    password: Some(msg.form.password),
                    role: "basic",
                })
                .await??;

            // notifie les autres systèmes
            self.events.send(UserRegistered(user_id));

            Ok(user_id)
        })
    }
}
