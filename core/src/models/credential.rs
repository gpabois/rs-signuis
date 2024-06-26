use uuid::Uuid;

use crate::Error;

pub struct Credential<'a> {
    pub name_or_email: &'a str,
    pub password: &'a str,
}

impl Credential<'_> {
    pub fn new(name_or_email: &str, password: &str) -> Self {
        Self {
            name_or_email,
            password,
        }
    }
}

/// Les identifiants stockés dans la base de données
pub struct HashedCredential {
    pub id: Uuid,
    pub password: String,
}

impl HashedCredential {
    pub fn verify_credential(&self, credential: &Credential) -> Result<(), Error> {
        let pwd_hash =
            password_hash::PasswordHash::new(&self.password).expect("invalid password hash");

        if let Result::Err(_) =
            pwd_hash.verify_password(&[&argon2::Argon2::default()], credential.password.clone())
        {
            return Error::invalid_credentials().into();
        }

        Ok(())
    }
}

#[derive(Default)]
/// Un filtre sur les informations d'identification.
pub struct CredentialFilter {
    pub name_or_email_eq: Option<String>,
}

impl CredentialFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name_or_email_equal_to(mut self, value: &str) -> Self {
        self.name_or_email_eq = Some(value.into());
        self
    }
}
