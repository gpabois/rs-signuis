use std::ops::Add;

use chrono::{Utc, Duration};
use futures::{future::BoxFuture, TryStreamExt};

use crate::{Error, model::{session::{Session, SessionFilter, Client}, credentials::{CredentialFilter, Credential}, session::InsertSession}, repositories::{sessions::traits::SessionRepository, credentials::traits::CredentialRepository}, drivers, utils::generate_token};

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{Error, model::{session::Session, credentials::Credential, session::Client}, drivers};

    pub trait Authentication {
        /// Verify the token, returns a session if the token is valid.
        fn check_session_token<'a, 'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, token: &'a str) -> BoxFuture<'b, Result<Session, Error>> where 'a : 'b;
        /// Get a session by an IP, creates one if does not exist.
        //fn get_or_create_session_by_ip<'a, 'b>(self, ip: &'a str) -> BoxFuture<'b, Result<Session, Error>> where 'a: 'b, 'q: 'b;
        /// Check credentials, and returns the underlying user id, if any.
        fn authenticate_by_credentials<'a, 'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, credential: Credential, client: &'a Client) -> BoxFuture<'b, Result<Session, Error>> where 'a: 'b;
    }
}

impl traits::Authentication for super::Service {
    fn check_session_token<'a, 'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, token: &'a str) -> BoxFuture<'b, Result<Session, Error>> where 'a : 'b {
        Box::pin(async {
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

    fn authenticate_by_credentials<'a, 'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, credential: Credential, client: &'a Client) -> BoxFuture<'b, Result<Session, Error>> where 'a: 'b {
        Box::pin(async {
            let scred_opt = self.repos.find_credentials_by(
                querier, 
                CredentialFilter::new().name_or_email_equal_to(&credential.name_or_email)
            )
            .try_next()
            .await?;

            if scred_opt.is_none() {
                return Err(Error::invalid_credentials())
            }

            let stored = scred_opt.unwrap();
            let pwd_hash = password_hash::PasswordHash::new(&stored.pwd_hash).expect("invalid password hash");
            
            // Wrong password, returns invalid_credentials error;
            if let Result::Err(_) = pwd_hash.verify_password(&[&argon2::Argon2::default()], credential.password) {
                return Error::invalid_credentials().into();
            }

            let token = generate_token(16);

            let mut insert = InsertSession::new(token, client.clone());
            insert.user_id = Some(stored.id);
            insert.expires_in = Utc::now().add(Duration::hours(8));

            self.repos.insert_session(querier, insert).await
            
        })
    }
}
