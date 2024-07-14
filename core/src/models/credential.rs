use argon2::PasswordHash;
use uuid::Uuid;

use crate::error::Error;

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// Identifiants stockés en BDD.
pub struct Credential {
    /// Identifiant du compte associé.
    pub id: Uuid,
    /// Mot de passe hashé.
    pub password: String,
}

impl Credential {
    /// Vérifie les identifiants par rapport à une soumission.
    pub fn verify(&self, password: &str) -> Result<bool, Error> {
        let pwd_hash = PasswordHash::new(&self.password).map_err(|_| Error::internal_error())?;

        Ok(pwd_hash
            .verify_password(&[&argon2::Argon2::default()], password)
            .is_ok())
    }
}
