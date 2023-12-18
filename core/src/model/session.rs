use chrono::Utc;
use sqlx::{FromRow, Row, postgres::PgRow};

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
    pub token:      String,
    pub client:     Client,
    pub user_id:    Option<String>,
    pub expires_in: chrono::DateTime<Utc>
}

impl InsertSession {
    pub fn new(token: String, client: Client) -> Self {
        Self{
            token,
            client,
            user_id: None,
            expires_in: Utc::now()    
        }
    }
}

pub struct Session {
    pub id:     String,
    pub client: Client,
    pub user:   Option<UserSession>,
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
