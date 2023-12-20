use sqlx::{postgres::PgRow, FromRow};

use crate::Error;

pub struct Credential {
    pub name_or_email: String,
    pub password: String
}

impl Credential {
    pub fn new(name_or_email: &str, password: &str) -> Self {
        Self{name_or_email: name_or_email.into(), password: password.into()}
    }
}

pub struct HashedCredential {
    pub id: String,
    pub pwd_hash: String   
}

impl<'r> FromRow<'r, PgRow> for HashedCredential {
    fn from_row(_row: &'r PgRow) -> Result<Self, sqlx::Error> {
        todo!()
    }
}

impl HashedCredential {
    pub fn verify_credential(&self, credential: Credential) -> Result<(), Error> {
        let pwd_hash = password_hash::PasswordHash::new(&self.pwd_hash).expect("invalid password hash");
        
        if let Result::Err(_) = pwd_hash.verify_password(&[&argon2::Argon2::default()], credential.password) {
            return Error::invalid_credentials().into();
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct CredentialFilter {
    pub name_or_email_eq: Option<String>
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
