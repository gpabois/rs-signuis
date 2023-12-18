pub struct Credential {
    pub name_or_email: String,
    pub password: String
}

pub struct HashedCredential {
    pub id: String,
    pub pwd_hash: String   
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
