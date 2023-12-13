pub struct Credentials {
    pub name_or_email: String,
    pub password: String
}

pub struct StoredCredentials {
    pub id: String,
    pub pwd_hash: String   
}