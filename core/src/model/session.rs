use chrono::Utc;
use sqlx::{FromRow, Row, postgres::PgRow};

use crate::utils::generate_token;

#[derive(Default)]
pub struct SessionFilter {
    pub token_eq:       Option<String>,
    pub expires_lte:    Option<chrono::DateTime<Utc>>,
    pub ip_eq:          Option<String>
}

impl SessionFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn token_equals(mut self, value: &str) -> Self {
        self.token_eq = Option::Some(value.into());
        self
    }

    pub fn expires_at_time_lower_or_equal(mut self, value: chrono::DateTime<Utc>) -> Self {
        self.expires_lte = Option::Some(value);
        self
    }
}

#[derive(Clone)]
pub struct Client {
    pub ip: String,
    pub user_agent: String
}

impl Client {
    pub fn new(ip: &str, user_agent: &str) -> Self {
        Self{
            ip: ip.into(),
            user_agent: user_agent.into()
        }
    }
}

pub type SessionClient = Client;

impl<'r> FromRow<'r, PgRow> for SessionClient {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            ip: row.try_get("client_ip")?,
            user_agent: row.try_get("client_user_agent")?
        })
    }
}

pub struct InsertSession {
    pub id:         Option<String>,
    pub token:      String,
    pub client:     Client,
    pub user_id:    Option<String>,
    pub expires_in: chrono::DateTime<Utc>
}

impl InsertSession {
    pub fn new_with_token(token: String, client: Client) -> Self {
        Self{
            id: None,
            token,
            client,
            user_id: None,
            expires_in: Utc::now()    
        }
    }

    pub fn new(client: Client) -> Self {
        let token = generate_token(16);
        Self{
            id: None,
            token,
            client,
            user_id: None,
            expires_in: Utc::now()    
        }
    }

    pub fn set_id(mut self, value: &str) -> Self {
        self.id = Some(value.into());
        self       
    }

    pub fn set_user_id(mut self, user_id: &str) -> Self {
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

pub struct Session {
    pub id:     String,
    pub client: Client,
    pub user:   Option<UserSession>,
}

impl Session {
    /// Create an anonymous session
    pub fn anonymous(client: Client) -> Self {
        Self {
            id: "anonymous".into(),
            user: None,
            client
        }
    }

    pub fn is_anonynmous(&self) -> bool {
        return self.user.is_none()
    }
}

pub struct UserSession {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>
}

struct OptionalUserSession(Option<UserSession>);

impl<'r> FromRow<'r, PgRow> for OptionalUserSession
{
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let user_id = row.try_get::<Option<String>, _>("user_id")?;
        Ok(match user_id {
            None => OptionalUserSession(Option::None),
            Some(id) => OptionalUserSession(Some(UserSession {
                id,
                name: row.get("user_name"),
                email: row.get("user_email"),
                avatar: row.get("user_avatar")
            }))
        })
    }
}

impl Into<Option<UserSession>> for OptionalUserSession {
    fn into(self) -> Option<UserSession> {
        self.0
    }
}

impl<'r> FromRow<'r, PgRow> for Session 
{
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Result::Ok(Session {
            id: row.try_get("session_id")?,
            client: SessionClient::from_row(row)?,
            user: OptionalUserSession::from_row(row)?.into()
        })
    }
}
