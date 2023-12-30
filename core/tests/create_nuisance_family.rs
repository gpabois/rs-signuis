use signuis_core::fixtures::{self};
use signuis_core::services::reporting::traits::Reporting;
mod setup;

#[tokio::test]
async fn create_nuisance_family_with_valid_values() -> Result<(), signuis_core::Error> {
    setup::with_service(|tx| {
        Box::pin(async {
            let args = fixtures::nuisance_families::NuisanceFamilyFixture::new();
            let actor = fixtures::sessions::new_anonymous_session();
            tx.create_nuisance_family(args, &actor).await?;
            Ok(())
        })
    }).await
}   