use sqlx::{FromRow, Row};

use super::user::User;

pub struct Session {
    pub id:     String,
    pub user:   Option<User>,
    pub ip:     String
}

impl<'r, R> FromRow<'r, R> for Session where R: Row {
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        Result::Ok(Session {
            id: row.try_get("session_id")?,
            ip: row.try_get("session_ip")?,
            user: row.try_get::<Option<String>, &str>("user_id")?.map(|_| {
                return User {
                    id: row.get("user_id"),
                    name: row.get("user_name"),
                    email: row.get("user_email"),
                    avatar: row.get("user_avatar")
                }
            })
        })
    }
}
