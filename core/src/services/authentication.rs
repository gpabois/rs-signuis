use chrono::{Utc, Duration};
use sea_query::{Query, PostgresQueryBuilder, CommonTableExpression, Alias, Expr};
use sea_query_binder::SqlxBinder;
use sqlx::{Pool, Postgres, Row, FromRow};
use async_trait::async_trait;

use crate::{Error, model::{session::Session, user::UserFilter, auth::Credentials}, sql::{UserIden, SessionIden}, utils::generate_token};

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
        return Result::Ok(Option::Some(Session::from_row(&row)?));

    }

    async fn get_session_by_ip(&self, ip: &String) -> Result<Session, Error> {
        let result= sqlx::query("SELECT id, user_id, ip FROM Session as t0 WHERE t0.ip=?")
            .bind(ip)
            .fetch_optional(&self.pool)
            .await?;

        if result.is_some() {
            let row = result.unwrap();
            return Result::Ok(Session::from_row(&row)?);
        }   

        return self.create_session_by_ip(ip).await;
    }
}

impl Authentication {
    async fn create_session_by_user(&self, user_id: String, ip: String) -> Result<Session, Error> {
        let token = generate_token(16);

        let mut expires_at = Utc::now().checked_add_signed(Duration::hours(8)).unwrap();
        
        let (sql, arguments) = Query::with().cte(
            CommonTableExpression::new()
                .table_name(Alias::new("inserted_report"))
                .query(Query::insert()
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
                        ])
                        .expect("Cannot bind values")
                        .returning(Query::returning()
                            .columns([
                                SessionIden::ID, 
                                SessionIden::UserID, 
                                SessionIden::IP
                            ])
                        )
                        .to_owned()
                ).to_owned()
        ).query(
            Query::select()
                .from(Alias::new("inserted_report"))
                .left_join(UserIden::Table, Expr::col((UserIden::Table, UserIden::ID)).equals(Alias::new("id")))
                .exprs([
                    Expr::col(SessionIden::ID).as_enum(Alias::new("session_id")),
                    Expr::col(SessionIden::IP).as_enum(Alias::new("session_ip")),
                    Expr::col(UserIden::ID).as_enum(Alias::new("user_id")),
                    Expr::col(UserIden::Name).as_enum(Alias::new("user_name")),
                    Expr::col(UserIden::Email).as_enum(Alias::new("user_email")),
                    Expr::col(UserIden::Avatar).as_enum(Alias::new("user_avatar")),
                ])
                .to_owned()
        ).build_sqlx(PostgresQueryBuilder);
        
        let row = sqlx::query_with(&sql, arguments).fetch_one(&self.pool).await?;
        return Result::Ok(Session::from_row(&row)?);
    }
    
    /// Create a session based on the IP
    async fn create_session_by_ip(&self, ip: &String) -> Result<Session, Error> {
        let (sql, arguments) = Query::insert()
            .into_table(SessionIden::Table)
            .columns([SessionIden::IP])
            .values([ip.into()])
            .expect("cannot bind values")
            .returning(Query::returning().columns([SessionIden::ID, SessionIden::UserID, SessionIden::IP]))
            .to_owned()
            .build_sqlx(PostgresQueryBuilder);

        let row = sqlx::query_with(&sql, arguments)
            .fetch_one(&self.pool)
            .await?;

        return Result::Ok(Session::from_row(&row)?);
    }
}