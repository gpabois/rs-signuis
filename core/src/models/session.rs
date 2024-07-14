use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user::UserRole;

pub type UserSessionId = Uuid;

#[derive(Clone)]
pub enum Session {
    Anonymous,
    User(UserSession),
}

impl Session {
    /// Retourne l'utilisateur derrière la session, si il existe.
    pub fn user(&self) -> Option<&SessionUser> {
        match self {
            Self::User(session) => Some(&session.user),
            _ => None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(::sqlx::FromRow))]
/// Session utilisateur
pub struct UserSession {
    pub id: Uuid,
    #[cfg_attr(feature = "sqlx", sqlx(flatten))]
    pub user: SessionUser,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(::sqlx::FromRow))]
pub struct SessionUser {
    #[cfg_attr(feature = "sqlx", sqlx(rename = "user_id"))]
    pub id: Uuid,

    #[cfg_attr(feature = "sqlx", sqlx(rename = "user_username"))]
    pub username: String,

    #[cfg_attr(feature = "sqlx", sqlx(rename = "user_email"))]
    pub email: String,

    #[cfg_attr(feature = "sqlx", sqlx(rename = "user_avatar"))]
    pub avatar: Option<String>,

    #[cfg_attr(feature = "sqlx", sqlx(rename = "user_role"))]
    pub role: UserRole,
}
