use chrono::{DateTime, Utc};
use node_bindgen::core::JSValue;
use node_bindgen::derive::node_bindgen;

use crate::types::uuid::Uuid;
use crate::utils::generate_token;

use super::client::Client;


/// Permet de filtrer une liste de session utilisateur
pub enum SessionFilter {
    TokenEq(String),
    ExpiresAtLte(DateTime<Utc>),
    ExpiresAtGte(DateTime<Utc>),
    And(Vec<SessionFilter>),
    Or(Vec<SessionFilter>),
}

impl SessionFilter {
    pub fn and<I: IntoIterator<Item = Self>>(values: I) -> Self {
        Self::And(values.into_iter().collect())
    }

    pub fn or<I: IntoIterator<Item = Self>>(values: I) -> Self {
        Self::Or(values.into_iter().collect())
    }

    pub fn token_equals(value: &str) -> Self {
        Self::TokenEq(value.into())
    }

    pub fn expires_at_time_lower_or_equal(value: chrono::DateTime<Utc>) -> Self {
        Self::ExpiresAtLte(value)
    }
}

pub type SessionClient = Client;

/// Objet pour créer une nouvelle session utilisateur.
pub struct NewUserSession {
    pub token: String,
    pub user_id: Uuid,
    pub expires_in: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
}

/// Objet pour insérer une nouvelle session utilisateur.
pub struct InsertUserSession {
    pub id: Option<Uuid>,
    pub token: String,
    pub user_id: Uuid,
    pub expires_in: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<NewUserSession> for InsertUserSession {
    fn from(value: NewUserSession) -> Self {
        Self {
            id: None,
            token: value.token,
            user_id: value.user_id,
            expires_in: value.expires_in,
            created_at: value.created_at
        }
    }
}

impl InsertUserSession {
    pub fn new_with_token(user_id: Uuid, token: String) -> Self {
        Self {
            id: None,
            token,
            user_id,
            expires_in: Utc::now(),
            created_at: None,
        }
    }

    pub fn new(user_id: Uuid) -> Self {
        let token = generate_token(16);
        Self {
            id: None,
            token,
            user_id,
            expires_in: Utc::now(),
            created_at: None,
        }
    }

    pub fn set_id<I: Into<Uuid>>(mut self, value: I) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn set_expires_in(mut self, value: chrono::DateTime<Utc>) -> Self {
        self.expires_in = value;
        self
    }

    pub fn set_token(mut self, value: &str) -> Self {
        self.token = value.into();
        self
    }
}


/// Session utilisateur
pub struct UserSession {
    pub id: Uuid,
    pub user: SessionUser,
    pub token: String,
}

#[derive(Clone)]
pub struct SessionUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
}
