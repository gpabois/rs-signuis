use std::ops::Add;

use chrono::{Utc, Duration};
use signuis_core::{model::session::InsertSession, services::authentication::traits::Authentication};

mod setup;
mod fixtures;

use fixtures::Fixture;

#[tokio::test]
async fn check_session_token_with_valid_token() -> Result<(), signuis_core::Error> {
    // Fixtures
    let token = "token1234";

    setup::with_service(|tx| {
        Box::pin(async move{
            let session = fixtures::sessions::SessionFixture::new()
                                .with_token(token)
                                .into_entity(tx)
                                .await?;
            
            let stored = tx.check_session_token(token).await?;

            assert_eq!(session.id, stored.id);

            Ok(())
        })
    }).await
}

#[tokio::test]
async fn check_session_token_with_invalid_token() -> Result<(), signuis_core::Error> {
    // Fixtures
    let token = "token1234";

    setup::with_service(|tx| {
        Box::pin(async move{
            let session = fixtures::sessions::SessionFixture::new()
                                .with_token(token)
                                .into_entity(tx)
                                .await?;
            
            let result = tx.check_session_token("fake_token").await;

            assert_eq!(result.is_err(), true);
    
            Ok(())
        })
    }).await
}

#[tokio::test]
async fn check_session_token_with_expired_session() -> Result<(), signuis_core::Error> {
    // Fixtures
    let token = "token1234";

    setup::with_service(|tx| {
        Box::pin(async move{
            // Generate an expired session
            fixtures::sessions::SessionFixture::new()
                .with_token(token)
                .with_expires_at(Utc::now().add(Duration::hours(-10)))
                .into_entity(tx)
                .await?;
            
            let result = tx.check_session_token(token).await;
            assert_eq!(result.is_err(), true);
    
            Ok(())
        })
    }).await
}