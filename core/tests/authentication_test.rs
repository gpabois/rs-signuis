use signuis_core::{Error, services::traits::Authentication};

mod setup;

#[tokio::test]
async fn test_authentication_get_session_by_ip() -> Result<(), Error> {
    setup::setup_config();
    let hub = setup::setup_services().await?;
    let mut tx = hub.begin().await?;

    let ip = "192.168.1.1";
    let session = tx.get_session_by_ip(ip).await?;

    assert_eq!(session.id, "192.168.1.1");
    assert_eq!(session.user.is_none(), true);

    Ok(())
}
