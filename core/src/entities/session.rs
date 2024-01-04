use chrono::{Utc, DateTime};
use node_bindgen::core::JSValue;
use node_bindgen::derive::node_bindgen;

use crate::types::uuid::Uuid;
use crate::utils::generate_token;

use super::client::Client;
pub enum SessionFilter {
    TokenEq(String),
    ExpiresAtLte(DateTime<Utc>),
    ExpiresAtGte(DateTime<Utc>),
    And(Vec<SessionFilter>),
    Or(Vec<SessionFilter>),
}

impl SessionFilter 
{
    pub fn and<I: IntoIterator<Item=Self>>(values: I) -> Self {
        Self::And(values.into_iter().collect())
    }

    pub fn or<I: IntoIterator<Item=Self>>(values: I) -> Self {
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

pub struct InsertSession {
    pub id:         Option<Uuid>,
    pub token:      String,
    pub client:     SessionClient,
    pub user_id:    Option<Uuid>,
    pub expires_in: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>
}

impl InsertSession {
    pub fn new_with_token(token: String, client: Client) -> Self {
        Self{
            id: None,
            token,
            client,
            user_id: None,
            expires_in: Utc::now(),
            created_at: None  
        }
    }

    pub fn new(client: Client) -> Self {
        let token = generate_token(16);
        Self{
            id: None,
            token,
            client,
            user_id: None,
            expires_in: Utc::now(),
            created_at: None  
        }
    }

    pub fn set_id<I: Into<Uuid>>(mut self, value: I) -> Self {
        self.id = Some(value.into());
        self       
    }

    pub fn set_user_id<I: Into<Uuid>>(mut self, user_id: I) -> Self {
        self.user_id = Some(user_id.into());
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

#[node_bindgen]
pub struct Session {
    pub id:     Option<Uuid>,
    pub client: Client,
    pub user:   Option<SessionUser>,
    pub token: String
}

impl JSValue<'_> for Session {
    fn convert_to_rust(env: &'_ node_bindgen::core::val::JsEnv, js_value: node_bindgen::sys::napi_value) -> Result<Self, node_bindgen::core::NjError> {
        todo!()
    }
}

impl Session {
    /// Create an anonymous session
    pub fn anonymous(client: Client) -> Self {
        Self {
            id: None,
            user: None,
            client,
            token: "".into()
        }
    }

    pub fn is_anonynmous(&self) -> bool {
        return self.user.is_none()
    }
}

#[node_bindgen]
#[derive(Clone)]
pub struct SessionUser {
    pub id:     Uuid,
    pub name:   String,
    pub email:  String,
    pub avatar: Option<String>
}
