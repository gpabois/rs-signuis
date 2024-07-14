//! Actions are requests handled by the actix framework.
pub mod auth;

pub use auth::authenticate_with_credential;
