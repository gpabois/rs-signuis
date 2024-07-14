use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CredentialForm {
    pub username_or_email: String,
    pub password: String,
}

impl CredentialForm {
    pub fn new(name_or_email: &str, password: &str) -> Self {
        Self {
            username_or_email: name_or_email.to_string(),
            password: password.to_string(),
        }
    }
}
