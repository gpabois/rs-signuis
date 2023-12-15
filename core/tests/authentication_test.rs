use signuis_core::{Error, services::traits::{Authentication, Container}};
use sqlx::Executor;

mod setup;

#[tokio::test]
async fn test_authentication_get_session_by_ip() -> Result<(), Error> {
    setup::setup_config();
    
    let hub = setup::setup_services().await?;
    let auth = Container<Authentication>::with(&hub);
    //.get_session_by_ip("192.168.1.1").await?;

    Ok(())
}