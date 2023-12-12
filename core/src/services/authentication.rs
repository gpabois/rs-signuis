use chrono::{Utc, DurationRound, Duration};
use sea_query::{Query, PostgresQueryBuilder};
use sea_query_binder::SqlxBinder;
use sqlx::{Pool, Postgres, Row};
use async_trait::async_trait;

use crate::{Error, model::{session::Session, user::{User, UserFilter}, auth::Credentials}, sql::{UserIden, SessionIden}, utils::generate_token};

use super::account::filter_user;

#[async_trait]
pub trait IAuthentication {
    /// Verify the token
    async fn check_token(&self, token: String) -> Result<Option<Session>, Error>;
    /// Get a session by an IP, creates one if does not exist.
    async fn get_session_by_ip(&self, ip: &String) -> Result<Session, Error>;
    /// Check credentials, and creates a session
    async fn check_credentials(&self, credentials: Credentials, ip: String) -> Result<Option<Session>, Error>;
}

pub struct Authentication {
    pool: Pool<Postgres>
}

#[async_trait]
impl IAuthentication for Authentication {
    async fn check_credentials(&self, credentials: Credentials, ip: String) -> Result<Option<Session>, Error> {
        let mut qb = Query::select();
        
        qb
        .from(UserIden::Table)
        .columns([UserIden::ID, UserIden::Password]);

        filter_user(
            &mut qb, 
            UserFilter::new()
                .name_or_email(credentials.name_or_email)
        );

        let (sql, arguments) = qb.build_sqlx(PostgresQueryBuilder);

        let result = sqlx::query_with(&sql, arguments)
        .fetch_optional(&self.pool)
        .await?;
        
        if result.is_none() {
            return Result::Ok(Option::None);    
        }

        let user_row = result.unwrap();
        let hash: String = user_row.get("password");
        let pwd_hash = password_hash::PasswordHash::new(&hash)
        .expect("invalid password hash");

        // Wrong password, returns invalid_credentials error;
        if let Result::Err(_) = pwd_hash.verify_password(&[&argon2::Argon2::default()], credentials.password) {
            return Error::invalid_credentials().into();
        }

        let user_id: String = user_row.get("id");
        let session = self.create_session_by_user(user_id, ip).await?;

        return Result::Ok(Option::Some(session));

    }
    
    async fn check_token(&self, token: String) -> Result<Option<Session>, Error> {
        let result = sqlx::query("
            SELECT 
                t0.id       AS session_id, 
                t0.ip       AS session_ip,
                t1.id       AS user_id, 
                t1.name     AS user_name, 
                t1.email    AS user_email,
                t1.avatar   AS user_avatar,
            FROM Session as t0
            LEFT JOIN User AS t1 ON t1.id == t0.user_id
            WHERE t0.token = ? AND t0.expires_at > NOW()
        ")
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;
        
        if result.is_none() {
            return Result::Ok(Option::None);
        }

        let row = result.unwrap();
        
        let session = Session {
            id: row.get("session_id"),
            ip: row.get("session_ip"),
            user: row.get::<Option<String>, &str>("user_id").map(|_| {
                return User {
                    id: row.get("user_id"),
                    name: row.get("user_name"),
                    email: row.get("user_email"),
                    avatar: row.get("user_avatar")
                }
            })
        };

        return Result::Ok(Option::Some(session));

    }

    async fn get_session_by_ip(&self, ip: &String) -> Result<Session, Error> {
        let result: Option<(String, String)> = sqlx::query_as("SELECT id, ip FROM Session as t0 WHERE t0.ip=?")
            .bind(ip)
            .fetch_optional(&self.pool)
            .await?;

        if result.is_some() {
            let row = result.unwrap();
            return Result::Ok(Session::from_ip(row));
        }   

        return self.create_session_by_ip(ip).await;
    }
}

impl Authentication {
    async fn create_session_by_user(&self, user_id: String, ip: String) -> Result<Session, Error> {
        let token = generate_token(16);

        let mut expires_at = Utc::now().checked_add_signed(Duration::hours(8)).unwrap();
        let mut qb = Query::insert();
        
        let (sql, arguments) = qb
        .into_table(SessionIden::Table)
        .columns([
            SessionIden::UserID,
            SessionIden::IP,
            SessionIden::Token,
            SessionIden::ExpiresAt
        ]).values([
            user_id.into(),
            ip.into(),
            token.into(),
            expires_at.into()
        ]).build_sqlx(PostgresQueryBuilder);
        

    }
    /// Create a session based on the IP
    async fn create_session_by_ip(&self, ip: &String) -> Result<Session, Error> {
        let row: (String, String) = sqlx::query_as("INSERT INTO Session (ip) VALUES (?) RETURNING (id, ip)")
        .bind(ip)
        .fetch_one(&self.pool)
        .await?;

        return Result::Ok(Session::from_ip(row));
    }
}