use chrono::DateTime;
use futures::{future::BoxFuture, TryStreamExt};
use sqlx::{Row, FromRow, Executor, Postgres};

use crate::{Error, model::{session::{Session, SessionFilter}, credentials::{Credentials, StoredCredentials, CredentialFilter, Credential}}, repositories::{sessions::traits::SessionRepository, credentials::traits::CredentialRepository}};

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{Error, model::{session::Session, credentials::Credentials}, repositories::sessions::traits::SessionRepository};

    pub trait Authentication {
        /// Verify the token, returns a session if the token is valid.
        fn check_token<'b, Q: driver::DatabaseQuerier>(self, querier: Q, token: &'a str) -> BoxFuture<'b, Result<Session, Error>> where 'a : 'b;
        /// Get a session by an IP, creates one if does not exist.
        //fn get_or_create_session_by_ip<'a, 'b>(self, ip: &'a str) -> BoxFuture<'b, Result<Session, Error>> where 'a: 'b, 'q: 'b;
        /// Check credentials, and returns the underlying user id, if any.
        fn check_credentials<'a, 'b, Q: driver::DatabaseQuerier>(self, querier: Q, credentials: Credentials, ip: &'a str) -> BoxFuture<'b, Result<String, Error>> where 'a: 'b;
        /// Create a session
        //fn create_session<'a>(self, args: CreateSession) -> BoxFuture<'b, Result<Session, Error>> where 'a: 'b, 'q: 'b;
    }
}

impl traits::Authentication for super::Service {
    fn check_token<'b, Q: driver::DatabaseQuerier>(self, querier: Q, token: &'a str) -> BoxFuture<'b, Result<Session, Error>> where 'a : 'b {
        Box::pin(async {
            let filter = SessionFilter::new()
                .token_equals(token)
                .expires_at_time_lower_or_equal(Utc::now());

            self.repos.find_session_by(querier, filter).try_next()
        })
    }

    fn check_credentials<'a, 'b, Q: driver::DatabaseQuerier>(self, querier: Q, credential: Credential, ip: &'a str) -> BoxFuture<'b, Result<String, Error>> where 'a: 'b {
        Box::pin(async {
            let scred_opt = self.repos.find_credentials_by(
                querier, 
                CredentialFilter::new().name_or_email_equal_to(&credential.name_or_email)
            )
            .try_next()
            .await?;

            if scred_opt.is_none() {
                return Err(Error::invalid_credentials())
            }

            
        })
    }
}

mod sql_query {
    use sea_query::{PostgresQueryBuilder, Query, CommonTableExpression, Alias, Expr};
    use sea_query_binder::{SqlxValues, SqlxBinder};

    use crate::{sql::{UserIden, SessionIden}, services::account::sql_query::filter_user, model::user::UserFilter, utils::generate_token};

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

    pub fn create_session_by_user_query(user_id: String, ip: &str) -> (String, SqlxValues) {
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

impl<'q, E> traits::Authentication<'q> for Authentication<&'q mut E> 
    where for<'a> &'a mut E: Executor<'a, Database = Postgres> + std::marker::Send
{
    fn check_credentials<'a, 'b>(self, credentials: Credentials, ip: &'a str) -> BoxFuture<'b, Result<Session, Error>> 
    where 'a: 'b, 'q: 'b
    {
       Box::pin(async  {
            check_credentials(self.db, credentials, ip).await
        })
    }
    
    fn check_token<'a, 'b>(self, token: &'a str) -> BoxFuture<'b, Result<Session, Error>> 
    where 'a: 'b, 'q: 'b
    {
        Box::pin(async {
            check_token(self.db, token).await
        })
    }

    fn get_session_by_ip<'a, 'b>(self, ip: &'a str) -> BoxFuture<'b, Result<Session, Error>> 
    where 'a: 'b, 'q: 'b
    {
        Box::pin(async {
            get_session_by_ip(self.db, ip).await          
        })
    }
}

/// Check the session token and returns a session if any
async fn check_token<'c, E: Executor<'c, Database = Postgres>>(executor: E, token: &str) -> Result<Session, Error> {
    let (sql, arguments) = sql_query::check_token_query(token);
    let query = sqlx::query_with(&sql, arguments);
    let result = executor.fetch_optional(query).await?;
    let row = result.ok_or(Error::InvalidTokenSession)?;
    let session = Session::from_row(&row)?;
    Ok(session)
}

// Check the credentials, returns the user id on success
async fn check_credentials_returns_user<'a, E>(executor: &'a mut E, credentials: Credentials) -> Result<String, Error> 
    where &'a mut E: Executor<'a, Database = Postgres>
{
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

async fn check_credentials<E>(executor: &mut E, credentials: Credentials, ip: &str) -> Result<Session, Error> 
    where for <'a> &'a mut E: Executor<'a, Database = Postgres>
{  

    let user_id = check_credentials_returns_user(
        executor, 
        credentials
    ).await?;


    let result = create_session_by_user(
        executor, 
        user_id, 
        ip).await;

    result
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

/// Create a user session
async fn create_session_by_user<'c, E>(executor: &'c mut E, user_id: String, ip: &str) -> Result<Session, Error> 
    where &'c mut E: Executor<'c, Database = Postgres>
{
    let (sql, arguments) = sql_query::create_session_by_user_query(user_id, ip);
    let row = sqlx::query_with(&sql, arguments).fetch_one(executor).await?;
    return Result::Ok(Session::from_row(&row)?);
}

/// Create an anonymous session
async fn create_session_by_ip<'c, E>(executor: E, ip: &str) -> Result<Session, Error> 
where E: Executor<'c, Database = Postgres>
{
    let (sql, arguments) = sql_query::create_session_by_ip_query(ip);

    let row = sqlx::query_with(&sql, arguments)
        .fetch_one(executor)
        .await?;

    return Result::Ok(Session::from_row(&row)?);
}

/// Find a session by its ip
async fn find_session_by_ip<'c, E>(executor: &'c mut E, ip: &str) -> Result<Option<Session>, Error> 
    where &'c mut E: Executor<'c, Database = Postgres>
{
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

/// Get session by ip
async fn get_session_by_ip<E>(conn: &mut E, ip: &str) -> Result<Session, Error> 
    where for<'a> &'a mut E: Executor<'a, Database = Postgres>
{
    let result = find_session_by_ip(conn, ip).await?;

    if result.is_some() {
        return Result::Ok(result.unwrap())
    }

    return create_session_by_ip(conn, ip).await;
}