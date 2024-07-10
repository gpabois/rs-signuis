pub struct CredentialForm {
    pub name_or_email: String,
    pub password: String,
}

impl CredentialForm {
    pub fn new(name_or_email: &str, password: &str) -> Self {
        Self {
            name_or_email: name_or_email.to_string(),
            password: password.to_string(),
        }
    }
}
