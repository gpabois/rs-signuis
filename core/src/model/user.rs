use chrono::Utc;
use sqlx::{prelude::FromRow, postgres::PgRow, Row};

pub struct RegisterUser {
    pub name:       String,
    pub email:      String,
    pub password:   String
}


#[derive(Default)]
pub struct CreateUser {
    pub name:       String,
    pub email:      String
}


#[derive(Default)]
pub struct UserFilter {
    pub name_or_email: Option<String>
}

/// A user filter
impl UserFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name_or_email<V: Into<String>>(mut self, value: V) -> Self {
        self.name_or_email = Option::Some(value.into());
        self
    }
}

pub struct User {
    pub id:     String,
    pub name:   String,
    pub email:  String,
    pub avatar: Option<String>
}

impl<'r> FromRow<'r, PgRow> for User {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            email: row.try_get("email")?,
            avatar: row.try_get("avatar")?
        })
    }
}
