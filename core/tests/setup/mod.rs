use std::error::Error;

use signuis_core::{
    models::{session::UserSession, user::UserRole},
    repositories::{user::fixtures::InsertUserFixture, BeginTx},
    SgSettings, Signuis,
};

/// Démarre le système Signuis avec un répertoire en mode transaction.
pub async fn setup() -> Result<Signuis, Box<dyn Error>> {
    let sg = Signuis::new(SgSettings::default().set_max_connections(1).to_owned()).await?;
    sg.repos.execute(BeginTx {}).await?;
    Ok(sg)
}

/// Crée une session avec le rôle d'administrateur.
pub async fn create_admin_session(sg: &Signuis) -> Result<UserSession, Box<dyn Error>> {
    let user = InsertUserFixture::default();
    user.role = UserRole::Administrator;

    let user_id = sg.repos.execute(user);
}

