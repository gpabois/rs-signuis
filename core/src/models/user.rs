use uuid::Uuid;

pub type UserId = uuid::Uuid;

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
