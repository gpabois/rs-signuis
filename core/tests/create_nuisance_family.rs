use std::error::Error;

use signuis_core::{
    forms::reporting::CreateNuisanceFamilyForm,
    models::{nuisance_family::CreateNuisanceFamily, session::Session},
};

mod setup;

#[tokio::test]
async fn create_nuisance_family_with_valid_values() -> Result<(), Box<dyn Error>> {
    let sg = setup::setup().await?;

    let nuisance_family = sg
        .reporting
        .execute(CreateNuisanceFamily {
            form: CreateNuisanceFamilyForm {
                label: "nuisance_type".to_owned(),
                description: "this is a nuisance family".to_owned(),
            },
            session: Session::Anonymous,
        })
        .await?;

    Ok(())
}

