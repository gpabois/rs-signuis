use chrono::{DateTime, Utc};
use uuid::Uuid;

pub type UserSessionId = Uuid;

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

/// Objet pour créer une nouvelle session utilisateur.
pub struct CreateUserSession {
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
}
