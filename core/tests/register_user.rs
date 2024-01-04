use signuis_core::fixtures::{self, Fixture};
use signuis_core::entities::user::RegisterUser;
use signuis_core::services::account::traits::Account;
mod setup;

#[tokio::test]
async fn register_user_with_valid_values() -> Result<(), signuis_core::Error> {
    setup::with_service(|tx| {
        Box::pin(async {
            let args = RegisterUser {
                name: "UserName".into(),
                email: "user@local.lan".into(),
                password: "1234".into(),
                confirm_password: "1234".into()
            };

            let actor = fixtures::sessions::new_anonymous_session();

            tx.register_user(args, &actor).await?;

            Ok(())
        })
    }).await
}   

#[tokio::test]
async fn register_user_with_existing_email() -> Result<(), signuis_core::Error> {
    setup::with_service(|tx| {
        Box::pin(async {
            fixtures::users::UserFixture::new().with_email("test@local.lan").into_entity(tx).await?;

            let args = RegisterUser {
                name: "UserName".into(),
                email: "test@local.lan".into(),
                password: "1234".into(),
                confirm_password: "1234".into()
            };

            let actor = fixtures::sessions::new_anonymous_session();

            let result = tx.register_user(args, &actor).await;
            assert_eq!(result.is_err(), true);

            Ok(())
        })
    }).await
}  


#[tokio::test]
async fn register_user_with_existing_name() -> Result<(), signuis_core::Error> {
    setup::with_service(|tx| {
        Box::pin(async {
            fixtures::users::UserFixture::new().with_name("UserName").into_entity(tx).await?;

            let args = RegisterUser {
                name: "UserName".into(),
                email: "test@local.lan".into(),
                password: "1234".into(),
                confirm_password: "1234".into()
            };

            let actor = fixtures::sessions::new_anonymous_session();

            let result = tx.register_user(args, &actor).await;
            assert_eq!(result.is_err(), true);

            Ok(())
        })
    }).await
}  

#[tokio::test]
async fn register_user_with_invalid_password() -> Result<(), signuis_core::Error> {
    setup::with_service(|tx| {
        Box::pin(async {
            let args = RegisterUser {
                name: "UserName".into(),
                email: "test@local.lan".into(),
                password: "1234".into(),
                confirm_password: "12345".into()
            };

            let actor = fixtures::sessions::new_anonymous_session();

            let result = tx.register_user(args, &actor).await;
            assert_eq!(result.is_err(), true);

            Ok(())
        })
    }).await
}   


#[tokio::test]
async fn register_user_with_invalid_email() -> Result<(), signuis_core::Error> {
    setup::with_service(|tx| {
        Box::pin(async {
            let args = RegisterUser {
                name: "UserName".into(),
                email: "invalid_email".into(),
                password: "1234".into(),
                confirm_password: "1234".into()
            };

            let actor = fixtures::sessions::new_anonymous_session();

            let result = tx.register_user(args, &actor).await;
            assert_eq!(result.is_err(), true);

            Ok(())
        })
    }).await
}   