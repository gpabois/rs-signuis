use leptos::{server, ServerFnError};
use signuis_core::models::nuisance_family::NuisanceFamily;

#[server]
pub async fn list_nuisance_families() -> Result<Vec<NuisanceFamily>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use signuis_core::{services::reporting::ListNuisanceFamilies, Signuis};

    let (sg,): (Data<Signuis>,) = extract().await?;

    let nuisance_families = sg.service.send(ListNuisanceFamilies::all()).await??;

    Ok(nuisance_families)
}
