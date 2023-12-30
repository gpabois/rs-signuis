use signuis_core::fixtures::{self};
use signuis_core::services::reporting::traits::Reporting;
mod setup;

#[tokio::test]
async fn create_nuisance_type_with_valid_values() -> Result<(), signuis_core::Error> {
    setup::with_service(|tx| {
        Box::pin(async {
            let args = fixtures::nuisance_types::NuisanceTypeFixture::new().create_deps(tx).await?.to_owned();
            let actor = fixtures::sessions::new_anonymous_session();
            tx.create_nuisance_type(args, &actor).await?;
            Ok(())
        })
    }).await
}   