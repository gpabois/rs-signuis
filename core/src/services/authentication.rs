use actix::{Handler, Message, ResponseFuture};
use chrono::{Duration, Utc};

use super::Service;
use crate::crypto::generate_token;
use crate::error::Error;
use crate::forms::authentication::CredentialForm;
use crate::repositories::credential::MaybeFindOneCredentialByNameOrEmail;
use crate::repositories::user_session::{
    InsertUserSession, MaybeFindOneValidUserSessionByToken, UserSession,
};

/// Authentifie avec des identifiants simples.
pub struct AuthenticateWithCredential {
    pub form: CredentialForm,
}

impl Message for AuthenticateWithCredential {
    type Result = Result<String, Error>;
}

/// Vérifie le jeton de la session utilisateur.
pub struct CheckUserSessionToken {
    pub token: String,
}

impl Message for CheckUserSessionToken {
    type Result = Result<UserSession, Error>;
}

impl Handler<AuthenticateWithCredential> for Service {
    type Result = ResponseFuture<Result<String, Error>>;

    fn handle(&mut self, msg: AuthenticateWithCredential, ctx: &mut Self::Context) -> Self::Result {
        let repos = self.repos.clone();

        Box::pin(async move {
            let cred = repos
                .send(MaybeFindOneCredentialByNameOrEmail(
                    msg.form.name_or_email.to_string(),
                ))
                .await??;

            cred.verify_credential(&msg.form.password)?;

            let user_id = cred.id;
            let token = generate_token(16);
            let expires_at = Utc::now().add(Duration::hours(8));

            self.repos
                .send(InsertUserSession {
                    user_id,
                    token,
                    expires_at,
                })
                .await?;
        })
    }
}

impl Handler<CheckUserSessionToken> for Service {
    type Result = Result<UserSession, Error>;

    fn handle(&mut self, msg: CheckUserSessionToken, ctx: &mut Self::Context) -> Self::Result {
        let repos = self.repos.clone();

        Box::pin(async {
            self.repos
                .send(MaybeFindOneValidUserSessionByToken(msg.token))
                .await??
                .ok_or_else(|| Error::invalid_user_session())
        })
    }
}
