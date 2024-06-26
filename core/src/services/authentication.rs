use std::{ops::Add, borrow::BorrowMut};
use chrono::{Utc, Duration};
use futures::{future::BoxFuture, TryStreamExt};
use sqlx::Acquire;
use crate::types::uuid::Uuid;

use crate::{Error, models::{user_session::{Session, SessionFilter}, credential::{CredentialFilter, Credential}, user_session::InsertSession}, repositories::{sessions::traits::SessionRepository, credential::traits::CredentialRepository}, Issues, Issue};

use super::{logger::{traits::Logger, logs::AuthenticationFailed}, authorization::{Action, traits::Authorization}};

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{Error, models::{user_session::Session, credential::Credential}};

    pub trait Authentication<'q> {
        /// Verify the token, returns a user session if the token is valid.
        fn check_session_token<'a, 'b>(self, token: &'a str) 
            -> BoxFuture<'b, Result<Session, Error>> 
        where 'a :'b, 'q: 'b;

        /// Check credentials, and returns a user session if valid.
        fn authenticate_with_credentials<'a, 'b, 'c>(self, credential: &'c Credential, actor: &'a Session) 
            -> BoxFuture<'b, Result<Session, Error>> 
        where 'a: 'b, 'c: 'b, 'q: 'b;
    }
}

impl<'q> traits::Authentication<'q> for &'q mut super::ServiceTx<'_> {
    fn check_session_token<'a, 'b>(self, token: &'a str) 
        -> BoxFuture<'b, Result<Session, Error>> 
    where 'a : 'b, 'q: 'b
    {
        Box::pin(async {
            let querier = self.querier.acquire().await?;

            let filter = SessionFilter::and(
                vec![
                    SessionFilter::TokenEq(token.into()),
                    SessionFilter::ExpiresAtGte(Utc::now())
                ]
            );

            self.repos
                .find_session_by(querier.borrow_mut(), filter)
                .try_next()
                .await?
                .ok_or(Error::invalid_credentials())
        })
    }

    fn authenticate_with_credentials<'a, 'b, 'c: 'b>(self, credential: &'c Credential, actor: &'a Session) 
        -> BoxFuture<'b, Result<Session, Error>> 
        where 'a : 'b, 'q: 'b {
        
        Box::pin(async {
            let mut issues = Issues::new();

            issues.async_assert_true(
                self.can(actor, Action::CanAuthenticate), 
                Issue::new_invalid_form( 
                "Nombre maximal d'authentifications infructueuses atteint, veuillez attendre une quinzaine de minutes.",
                ["*"]
                )
            ).await?;

            let result: Result<Uuid, Error> = {
                let querier = self.querier.acquire().await?;

                let invalid_credential = Issue::new_invalid_form( 
                    "Les données d'identification sont invalides",
                    ["user_or_email"]
                );
    
                let stored = issues.assert_some(
                    self.repos.find_one_credential_by(&mut *querier,
                    CredentialFilter::new()
                            .name_or_email_equal_to(&credential.name_or_email)
                    )
                    .await?, 
                    invalid_credential.clone()
                )?;
    
                issues.assert_success(
                    stored.verify_credential(credential), 
                    invalid_credential.clone()
                )?;

                Ok(stored.id)
            };

            if result.is_err() {
                self.log(AuthenticationFailed::new(actor)).await?;
            }

            let user_id = result?;

            let querier = self.querier.acquire().await?;
            let insert = InsertSession::new(actor.client.clone())
                .set_user_id(user_id)
                .set_expires_in(
                    Utc::now()
                    .add(Duration::hours(8))
                );

            self.repos.insert_session(querier, insert).await
        })
    }
}
