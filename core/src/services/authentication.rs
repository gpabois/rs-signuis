use std::ops::Add;

use chrono::{Utc, Duration};
use futures::{future::BoxFuture, TryStreamExt};
use sqlx::Acquire;

use crate::{Error, model::{session::{Session, SessionFilter, Client}, credentials::{CredentialFilter, Credential}, session::InsertSession}, repositories::{sessions::traits::SessionRepository, credentials::traits::CredentialRepository}};

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{Error, model::{session::Session, credentials::Credential}};

    pub trait Authentication<'q> {
        /// Verify the token, returns a session if the token is valid.
        fn check_session_token<'a, 'b>(self, token: &'a str) 
            -> BoxFuture<'b, Result<Session, Error>> where 'a :'b, 'q: 'b;

        /// Get a session by an IP, creates one if does not exist.
        //fn get_or_create_session_by_ip<'a, 'b>(self, ip: &'a str) -> BoxFuture<'b, Result<Session, Error>> where 'a: 'b, 'q: 'b;
        /// Check credentials, and returns the underlying user id, if any.
        fn authenticate_with_credentials<'a, 'b>(self, credential: Credential, actor: &'a Session) 
            -> BoxFuture<'b, Result<Session, Error>> where 'a: 'b, 'q: 'b;
    }
}

impl<'q> traits::Authentication<'q> for &'q mut super::ServiceTx<'_>
{
    fn check_session_token<'a, 'b>(self, token: &'a str) 
        -> BoxFuture<'b, Result<Session, Error>> where 'a : 'b, 'q: 'b
    {
        Box::pin(async {
            let querier = self.querier.acquire().await?;

            let filter = SessionFilter::new()
                .token_equals(token)
                .expires_at_time_lower_or_equal(Utc::now());

            self.repos
                .find_session_by(querier, filter)
                .try_next()
                .await?
                .ok_or(Error::invalid_credentials())
        })
    }

    fn authenticate_with_credentials<'a, 'b>(self, credential: Credential, actor: &'a Session) 
        -> BoxFuture<'b, Result<Session, Error>> 
        where 'a : 'b, 'q: 'b {
        
        Box::pin(async {
            let querier = self.querier.acquire().await?;

            let scred_opt = self.repos.find_credentials_by(
                &mut *querier, // Explicit reborrow
                CredentialFilter::new().name_or_email_equal_to(&credential.name_or_email)
            )
            .try_next()
            .await?;

            if scred_opt.is_none() {
                return Err(Error::invalid_credentials())
            }

            let stored = scred_opt.unwrap();
            stored.verify_credential(credential)?;

            let insert = InsertSession::new(actor.client.clone())
                .set_user_id(&stored.id)
                .set_expires_in(
                    Utc::now()
                        .add(Duration::hours(8)
                    )
                );

            self.repos.insert_session(querier, insert).await
        })
    }
}
