mod setup;

use signuis_core::{
    forms::authentication::CredentialForm, models::session::Session,
    repositories::user::fixtures::InsertUserFixture,
    services::authentication::AuthenticateWithCredential,
};
use std::error::Error;

#[tokio::test]
async fn authenticate_with_credentials_with_name_and_valid_password() -> Result<(), Box<dyn Error>>
{
    let sg = setup::setup().await?;

    let fixture = InsertUserFixture::new();
    let user_id = sg.repos.execute(fixture.clone()).await?;

    let session = sg
        .auth
        .execute(AuthenticateWithCredential {
            form: CredentialForm {
                username_or_email: fixture.username.clone(),
                password: fixture.password.unwrap(),
            },
            session: Session::Anonymous,
        })
        .await?;

    assert_eq!(user_id, session.user_id);
    Ok(())
}

#[tokio::test]
async fn authenticate_with_credentials_with_email_and_valid_password() -> Result<(), Box<dyn Error>>
{
    let sg = setup::setup().await?;

    let fixture = InsertUserFixture::new();
    let user_id = sg.repos.execute(fixture.clone()).await?;

    let session = sg
        .auth
        .execute(AuthenticateWithCredential {
            form: CredentialForm {
                username_or_email: fixture.email.clone(),
                password: fixture.password.unwrap(),
            },
            session: Session::Anonymous,
        })
        .await?;

    assert_eq!(user_id, session.user_id);

    Ok(())
}

#[tokio::test]
async fn authenticate_with_credentials_with_invalid_password() -> Result<(), Box<dyn Error>> {
    let sg = setup::setup().await?;

    let fixture = InsertUserFixture::new();
    let user_id = sg.repos.execute(fixture.clone()).await?;

    let session = sg
        .auth
        .execute(AuthenticateWithCredential {
            form: CredentialForm {
                username_or_email: fixture.email.clone(),
                password: "wrong_password".into(),
            },
            session: Session::Anonymous,
        })
        .await?;

    assert_eq!(user_id, session.user_id);

    Ok(())
}
