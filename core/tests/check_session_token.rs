use chrono::{Duration, Utc};
use signuis_core::{
    repositories::{user::fixtures::InsertUserFixture, user_session::InsertUserSession},
    services::authentication::CheckUserSessionToken,
};
use std::error::Error;
use std::ops::Add;

mod setup;

#[tokio::test]
async fn check_session_token_with_valid_token() -> Result<(), Box<dyn Error>> {
    let sg = setup::setup().await?;

    let user_id = sg.repos.execute(InsertUserFixture::new()).await?;
    let session_id = sg
        .repos
        .execute(InsertUserSession {
            user_id,
            token: "token1234".to_owned(),
            expires_at: Utc::now().add(Duration::hours(1)),
        })
        .await?;

    let maybe_session = sg
        .auth
        .execute(CheckUserSessionToken {
            token: "token1234".to_owned(),
        })
        .await?;

    assert!(maybe_session.is_some());

    let session = maybe_session.unwrap();
    assert_eq!(session.id, session_id);

    Ok(())
}

#[tokio::test]
async fn check_session_token_with_invalid_token() -> Result<(), Box<dyn Error>> {
    let sg = setup::setup().await?;

    let user_id = sg.repos.execute(InsertUserFixture::new()).await?;
    sg.repos
        .execute(InsertUserSession {
            user_id,
            token: "token1234".to_owned(),
            expires_at: Utc::now().add(Duration::hours(1)),
        })
        .await?;

    let maybe_session = sg
        .auth
        .execute(CheckUserSessionToken {
            token: "wrong_token".to_owned(),
        })
        .await?;

    assert!(maybe_session.is_none());

    Ok(())
}

#[tokio::test]
async fn check_session_token_with_expired_session() -> Result<(), Box<dyn Error>> {
    let sg = setup::setup().await?;

    let user_id = sg.repos.execute(InsertUserFixture::new()).await?;
    sg.repos
        .execute(InsertUserSession {
            user_id,
            token: "token1234".to_owned(),
            expires_at: Utc::now().add(Duration::hours(-1)),
        })
        .await?;

    let maybe_session = sg
        .auth
        .execute(CheckUserSessionToken {
            token: "token1234".to_owned(),
        })
        .await?;

    assert!(maybe_session.is_none());

    Ok(())
}

