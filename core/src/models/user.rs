use crate::types::uuid::Uuid;
use argon2::Argon2;

/// Objet pour enregistrer un nouvel utilisateur.
pub struct RegisterUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub confirm_password: &'a str,
}

impl From<RegisterUser> for InsertUser {
    fn from(user: RegisterUser) -> Self {
        Self {
            id: None,
            name: user.name,
            email: user.email,
            password: Some(user.password.to_string()),
            role: None,
        }
    }
}

pub struct InsertUser<'a> {
    pub id: Option<Uuid>,
    pub name: &'a str,
    pub email: &'a str,
    pub password: Option<String>,
    pub role: Option<String>,
}

impl InsertUser {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            id: None,
            name: name.into(),
            email: email.into(),
            role: None,
            password: None,
        }
    }

    pub fn set_id<I: Into<Uuid>>(mut self, id: I) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn set_role(mut self, role: &str) -> Self {
        self.role = Some(role.into());
        self
    }

    pub fn set_password(mut self, password: &str) -> Self {
        self.password = Some(password.into());
        self.hash_password()
    }

    pub fn set_hashed_password(mut self, password: &str) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Hash the password
    pub fn hash_password(mut self) -> Self {
        match self.password {
            Some(pwd) => {
                let salt = password_hash::SaltString::generate(rand::thread_rng());
                self.password = Some(
                    password_hash::PasswordHash::generate(Argon2::default(), pwd, &salt)
                        .expect("cannot hash password")
                        .to_string(),
                );
                self
            }
            None => self,
        }
    }
}

pub enum UserFilter {
    Or(Vec<UserFilter>),
    And(Vec<UserFilter>),
    Name(String),
    Email(String),
}

/// A user filter
impl UserFilter {
    pub fn name_or_email<V: Into<String>>(value: V) -> Self {
        let v = value.into();

        Self::Or(vec![UserFilter::Name(v.clone()), UserFilter::Email(v)])
    }

    pub fn or(conds: Vec<UserFilter>) -> Self {
        Self::Or(conds)
    }

    pub fn name(value: &str) -> Self {
        Self::Name(value.into())
    }

    pub fn email(value: &str) -> Self {
        Self::Email(value.into())
    }
}

pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
}

impl Identifiable for User {
    type Type = Uuid;

    fn id(&self) -> Self::Type {
        self.id.clone()
    }
}
