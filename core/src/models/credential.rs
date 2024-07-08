use argon2::PasswordHash;
use uuid::Uuid;

use crate::error::Error;


pub struct CredentialSubmission<'a> {
    pub name_or_email: &'a str,
    pub password: &'a str,
}

impl CredentialSubmission<'_> {
    pub fn new(name_or_email: &str, password: &str) -> Self {
        Self {
            name_or_email,
            password,
        }
    }
}

#[derive(sqlx::FromRow)]
/// Identifiants stockés en BDD.
pub struct Credential {
    /// Identifiant du compte associé.
    pub id: Uuid,
    /// Mot de passe hashé.
    pub password: String
}

impl Credential {
    
    /// Vérifie les identifiants par rapport à une soumission.
    pub fn verify_against(&self, submission: &CredentialSubmission) -> Result<(), Error> {
        let pwd_hash =
            PasswordHash::new(&self.password).map_err(|source| Error::internal_error(source))?;

        pwd_hash
        .verify_password(&[&argon2::Argon2::default()], submission.password)
        .map_err(|_ |Error::invalid_credential())
    }
}