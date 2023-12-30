use signuis_core::fixtures::{self};
use signuis_core::services::reporting::traits::Reporting;
mod setup;

#[tokio::test]
async fn report_nuisance_with_valid_values() -> Result<(), signuis_core::Error> {
    setup::with_service(|tx| {
        Box::pin(async {
            let args = fixtures::nuisance_reports::NuisanceReportFixture::new().create_deps(tx).await?.to_owned();
            let actor = fixtures::sessions::new_anonymous_session();
            tx.report_nuisance(args, &actor).await?;
            Ok(())
        })
    }).await
}   