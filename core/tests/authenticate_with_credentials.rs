use std::ops::Add;

use chrono::Duration;
use chrono::Utc;
use signuis_core::fixtures::logs::LogFixture;
use signuis_core::entities::credentials::Credential;
use signuis_core::entities::session::Session;
use signuis_core::services::authentication::traits::Authentication;
use signuis_core::Error;

use signuis_core::fixtures;
use signuis_core::fixtures::Fixture;

mod setup;

const USER_ID: &str = "07483c5a-57f1-4f9a-a6f9-49cb961c1eff";
const USER_NAME: &str = "test_user";
const USER_EMAIL: &str = "test_user@local.lan";
const USER_PASSWORD: &str = "test_pwd";  

#[tokio::test]
async fn authenticate_with_credentials_with_name_and_valid_password() -> Result<(), Error> {
    setup::with_service(|tx| {
        Box::pin(async move {
            let client = fixtures::clients::new_client();

            // Create a fake user
            let user = fixtures::users::UserFixture::new()
                .with_id(USER_ID)
                .with_name(USER_NAME)
                .with_email(USER_EMAIL)
                .with_password(USER_PASSWORD)
                .into_entity(tx)
                .await?;

            // Check the credentials, and returns a session
            let session = tx.authenticate_with_credentials(
                &Credential::new(USER_NAME, USER_PASSWORD),
                &Session::anonymous(client)
            ).await?;

            assert_eq!(session.user.is_some(), true);

            let session_user = session.user.unwrap();
            // Check the user
            assert_eq!(session_user.id, user.id);
            assert_eq!(session_user.name, user.name);
            assert_eq!(session_user.email, user.email);
            
            Ok(())
        })
    }).await

}

#[tokio::test]
async fn authenticate_with_credentials_with_email_and_valid_password() -> Result<(), Error> {
    setup::with_service(|tx| {
        Box::pin(async move {
            let client = fixtures::clients::new_client();

            // Create a fake user
            let user = fixtures::users::UserFixture::new()
                .with_id(USER_ID)
                .with_name(USER_NAME)
                .with_email(USER_EMAIL)
                .with_password(USER_PASSWORD)
                .into_entity(tx)
                .await?;

            // Check the credentials, and returns a session
            let session = tx.authenticate_with_credentials(
                &Credential::new(USER_EMAIL, USER_PASSWORD),
                &Session::anonymous(client)
            ).await?;

            assert_eq!(session.user.is_some(), true);

            let session_user = session.user.unwrap();
            // Check the user
            assert_eq!(session_user.id, user.id);
            assert_eq!(session_user.name, user.name);
            assert_eq!(session_user.email, user.email);
            
            Ok(())
        })
    }).await

}

#[tokio::test]
async fn authenticate_with_credentials_with_invalid_password() -> Result<(), Error> {
    setup::with_service(|tx| {
        Box::pin(async move {
            let client = fixtures::clients::new_client();

            fixtures::users::new_user_with(
                &mut *tx,
                fixtures::users::UserFixture::new()
                    .with_id(USER_ID)
                    .with_name(USER_NAME)
                    .with_email(USER_EMAIL)
                    .with_password(USER_PASSWORD)
            ).await?;
    
            // Check the credentials, and returns a session
            let result = {
                tx.authenticate_with_credentials(
                    &Credential::new(USER_NAME, "wrong_password"),
                    &Session::anonymous(client)
                ).await
            };
        
            assert_eq!(result.is_err(), true);
            
            Ok(())
        })
    }).await
}

#[tokio::test]
async fn authenticate_with_credentials_with_no_password() -> Result<(), Error> {
    setup::with_service(|tx| {
        Box::pin(async move {
            let client = fixtures::clients::new_client();

            fixtures::users::new_user_with(
                &mut *tx,
                fixtures::users::UserFixture::new()
                    .with_id(USER_ID)
                    .with_name(USER_NAME)
                    .with_email(USER_EMAIL)
            ).await?;

            // Check the credentials, and returns a session
            let result = {
                tx.authenticate_with_credentials(
                    &Credential::new(USER_NAME, "wrong_password"),
                    &Session::anonymous(client)
                ).await
            };

            assert_eq!(result.is_err(), true);
            Ok(())
        })
    }).await
}


#[tokio::test]
async fn authenticate_with_reached_attempt() -> Result<(), Error> {
    setup::with_service(|tx| {
        Box::pin(async move {
            let client = fixtures::clients::new_client();

            fixtures::users::new_user_with(
                &mut *tx,
                fixtures::users::UserFixture::new()
                    .with_id(USER_ID)
                    .with_name(USER_NAME)
                    .with_email(USER_EMAIL)
                    .with_password(USER_PASSWORD)
            ).await?;

            // Make 3 attempts in less than 15 mn.
            let mut log = LogFixture::new()
            .with_client(client.clone())
            .with_type("authentication::failed")
            .to_owned();

            fixtures::logs::new_log_with(&mut *tx, log.with_at(Utc::now().add(Duration::minutes(-3))).to_owned()).await?;
            fixtures::logs::new_log_with(&mut *tx, log.with_at(Utc::now().add(Duration::minutes(-6))).to_owned()).await?;
            fixtures::logs::new_log_with(&mut *tx, log.with_at(Utc::now().add(Duration::minutes(-9))).to_owned()).await?;

            // Check the credentials, and returns a session
            let result = {
                tx.authenticate_with_credentials(
                    &Credential::new(USER_NAME, USER_PASSWORD),
                    &Session::anonymous(client)
                ).await
            };

            assert_eq!(result.is_err(), true);
            Ok(())
        })
    }).await
}
