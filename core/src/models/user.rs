use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type UserId = uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
    pub role: UserRole,
}

#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    User,
    Administrator,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::User
    }
}
