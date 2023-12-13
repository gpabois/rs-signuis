use sqlx::{Postgres, Row, FromRow, Executor};
use async_trait::async_trait;

use crate::{Error, model::{session::Session, auth::{Credentials, StoredCredentials}}};

#[async_trait]
pub trait IAuthentication {
    /// Verify the token
    async fn check_token(self, token: String) -> Result<Option<Session>, Error>;
    /// Get a session by an IP, creates one if does not exist.
    async fn get_session_by_ip(self, ip: &String) -> Result<Session, Error>;
    /// Check credentials, and creates a session
    async fn check_credentials(self, credentials: Credentials, ip: String) -> Result<Session, Error>;
}

pub struct AuthenticationArgs<A> {
    db: A
}

impl<A> AuthenticationArgs<A>{
    fn new(db: A) -> Self {
        Self{db}
    }
}

pub struct Authentication<A> {
    db: A
}

mod sql_query {
    use sea_query::{PostgresQueryBuilder, Query, CommonTableExpression, Alias, Expr};
    use sea_query_binder::{SqlxValues, SqlxBinder};

    use crate::{sql::{UserIden, SessionIden}, services::filter_user, model::user::UserFilter, utils::generate_token};

    pub fn check_credentials_query(name_or_email: &str) -> (String, SqlxValues) {
        let mut qb = Query::select()
        .from(UserIden::Table)
        .columns([UserIden::ID, UserIden::Password]).to_owned();
        
        filter_user(
            &mut qb, 
            UserFilter::new()
                .name_or_email(name_or_email)
        );   

        qb.build_sqlx(PostgresQueryBuilder)
    }

    pub fn create_session_by_user_query(user_id: String, ip: String) -> (String, SqlxValues) {
        let token = generate_token(16);

        //let mut expires_at = Utc::now().checked_add_signed(Duration::hours(8)).unwrap();
        
        Query::with().cte(
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
                            0.into()//expires_at.into()
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
        )
        .to_owned()
        .query(
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
        ).build_sqlx(PostgresQueryBuilder)
    }

    pub fn create_session_by_ip_query(ip: &str) -> (String, SqlxValues) {
        Query::insert()
            .into_table(SessionIden::Table)
            .columns([SessionIden::IP])
            .values([ip.into()])
            .expect("cannot bind values")
            .returning(Query::returning().columns([SessionIden::ID, SessionIden::UserID, SessionIden::IP]))
            .to_owned()
            .build_sqlx(PostgresQueryBuilder)
    }
}

#[async_trait]
impl<'c, A> IAuthentication for &'_ Authentication<A> 
    where for <'a> &'a A: Executor<'c, Database = Postgres> + std::marker::Send + std::marker::Sync, A : std::marker::Send + std::marker::Sync
{
    async fn check_credentials(self, credentials: Credentials, ip: String) -> Result<Session, Error> {
        let user_id: String = check_credentials(&self.db, credentials).await?;
        let session = create_session_by_user(&self.db, user_id, ip).await?;

        return Result::Ok(session);
    }
    
    async fn check_token(self, token: String) -> Result<Option<Session>, Error> {
        return check_token(&self.db, token).await;
    }

    async fn get_session_by_ip(self, ip: &String) -> Result<Session, Error> {
        let result = find_session_by_ip(&self.db, ip).await?;

        if result.is_some() {
            return Result::Ok(result.unwrap())
        }

        return create_session_by_ip(&self.db, ip).await;
    }
}

#[async_trait]
impl<'c, A> IAuthentication for &'_ mut Authentication<A> 
    where for<'a> &'a mut A: Executor<'c, Database = Postgres>, A : std::marker::Send {
    async fn check_credentials(self, credentials: Credentials, ip: String) -> Result<Session, Error> {
        let user_id: String = check_credentials(&mut self.db, credentials).await?;
        let session = create_session_by_user(&mut self.db, user_id, ip).await?;

        return Result::Ok(session);
    }
    
    async fn check_token(self, token: String) -> Result<Option<Session>, Error> {
        return check_token(&mut self.db, token).await;
    }

    async fn get_session_by_ip(self, ip: &String) -> Result<Session, Error> {
        let result = find_session_by_ip(&mut self.db, ip).await?;

        if result.is_some() {
            return Result::Ok(result.unwrap())
        }

        return create_session_by_ip(&mut self.db, ip).await;
    }
}

async fn check_token<'c, E: Executor<'c, Database = Postgres>>(executor: E, token: String) -> Result<Option<Session>, Error> {
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
    .fetch_optional(executor)
    .await?;

    if result.is_none() {
        return Result::Ok(Option::None);
    }

    let row = result.unwrap();
    return Result::Ok(Option::Some(Session::from_row(&row)?));
}

// Check the credentials, returns the user id on success
async fn check_credentials<'c, E: Executor<'c, Database = Postgres>>(executor: E, credentials: Credentials) -> Result<String, Error> {
    //
    let stored_opt = get_stored_credentials(executor, credentials.name_or_email).await?;

    if stored_opt.is_none() {
        return Error::invalid_credentials().into();
    }

    let stored = stored_opt.unwrap();
    let pwd_hash = password_hash::PasswordHash::new(&stored.pwd_hash).expect("invalid password hash");

    // Wrong password, returns invalid_credentials error;
    if let Result::Err(_) = pwd_hash.verify_password(&[&argon2::Argon2::default()], credentials.password) {
        return Error::invalid_credentials().into();
    }

    return Result::Ok(stored.id);
}

/// Get stored credentials
async fn get_stored_credentials<'c, E: Executor<'c, Database = Postgres>>(executor: E, name_or_email: String) -> Result<Option<StoredCredentials>, Error> {
    let (sql, arguments) = sql_query::check_credentials_query(&name_or_email);

    let result = sqlx::query_with(&sql, arguments)
    .fetch_optional(executor)
    .await?;
    
    if result.is_none() {
        return Result::Ok(Option::None);    
    }

    let row = result.unwrap();

    return Result::Ok(Option::Some(StoredCredentials {
        id: row.get("id"),
        pwd_hash: row.get("password")
    }))
}

/// Find a session by its ip
async fn find_session_by_ip<'c, E: Executor<'c, Database = Postgres>>(executor: E, ip: &str) -> Result<Option<Session>, Error> {
    let result= sqlx::query("SELECT id, user_id, ip FROM Session as t0 WHERE t0.ip=?")
    .bind(ip)
    .fetch_optional(executor)
    .await?;
    
    if result.is_some() {
        let row = result.unwrap();
        return Result::Ok(Option::Some(Session::from_row(&row)?));
    }   

    return Result::Ok(Option::None);
}

/// Create a user session
async fn create_session_by_user<'c, E: Executor<'c, Database = Postgres>>(executor: E, user_id: String, ip: String) -> Result<Session, Error> {
    let (sql, arguments) = sql_query::create_session_by_user_query(user_id, ip);
    let row = sqlx::query_with(&sql, arguments).fetch_one(executor).await?;
    return Result::Ok(Session::from_row(&row)?);
}

/// Create an anonymous session
async fn create_session_by_ip<'c, E: Executor<'c, Database = Postgres>>(executor: E, ip: &String) -> Result<Session, Error> {
    let (sql, arguments) = sql_query::create_session_by_ip_query(ip);

    let row = sqlx::query_with(&sql, arguments)
        .fetch_one(executor)
        .await?;

    return Result::Ok(Session::from_row(&row)?);
}

impl<A> Authentication<A> {
    pub fn new(args: AuthenticationArgs<A>) -> Self {
        Self {
            db: args.db
        }
    }
}