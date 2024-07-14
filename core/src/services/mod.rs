pub mod account;
pub mod authentication;
pub mod reporting;

use chrono::Duration;

#[derive(Clone)]
pub struct ServiceSettings {
    pub user_session_expiration_time: Duration,
}

impl Default for ServiceSettings {
    fn default() -> Self {
        Self {
            user_session_expiration_time: Duration::hours(8),
        }
    }
}
