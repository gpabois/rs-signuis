use sqlx::{FromRow, Row, postgres::PgRow};

pub struct SessionFilter {
    pub token_eq: Option<String>,
    pub ip_eq: Option<String>
}

pub struct Session {
    pub id:     String,
    pub ip:     String,
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
            ip: row.try_get("session_ip")?,
            user: OptionalUserSession::from_row(row)?.into()
        })
    }
}
