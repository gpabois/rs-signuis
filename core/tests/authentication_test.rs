use signuis_core::{Error, services::{Authentication, AuthenticationArgs, traits::Authentication as TraitAuth}};

mod setup;

#[tokio::test]
async fn test_authentication_get_session_by_ip() -> Result<(), Error> {
    setup::setup_config();
    let mut db = setup::setup_database().await?;
    
    let mut auth = Authentication::new(AuthenticationArgs::new(db));
    let session = auth.get_session_by_ip("192.168.1.1").await?;

    Result::Ok(())
}