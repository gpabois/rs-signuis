use futures::future::BoxFuture;
use sqlx::{Postgres, Row, Executor, FromRow};

use crate::{model::user::{RegisterUser, User, UserFilter}, Error};

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{model::user::{User, RegisterUser, UserFilter}, Error};


    pub trait Account<'q> {
        /// Register a new user
        fn register_user<'b>(self, args: RegisterUser) -> BoxFuture<'b, Result<User, Error>> where 'q: 'b;
        
        /// Count the number of users matching the filter
        fn count_user_by<'b>(self, filter: UserFilter) -> BoxFuture<'b, Result<i64, Error>> where 'q: 'b;
    }
}

pub struct Account<DB> {
    db: DB
}

impl<'q, DB> traits::Account<'q> for Account<&'q mut DB>
where for<'a> &'a mut DB: Executor<'a, Database = Postgres> + std::marker::Send
{
    fn count_user_by<'b>(self, filter: UserFilter) -> BoxFuture<'b, Result<i64, Error>> 
    where 'q: 'b
    {
        Box::pin(async {
            count_user_by(self.db, filter).await
        })
    }

    /// Register a new user
    fn register_user<'b>(self, args: RegisterUser) -> BoxFuture<'b, Result<User, Error>> 
    where 'q: 'b
    {
        Box::pin(async {
            register_user(self.db, args).await
        })
    }
}

async fn count_user_by<'c, E>(executor: &'c mut E, filter: UserFilter) -> Result<i64, Error> 
where &'c mut E: Executor<'c, Database = Postgres>
{
    let (sql, arguments) = sql_query::count_user_by_query(filter);
    let result = sqlx::query_with(&sql, arguments).fetch_one(executor).await?;
    Ok(result.get("count"))
}

/// Register a new user
async fn register_user<'c, E>(executor: &'c mut E, args: RegisterUser) -> Result<User, Error> 
    where &'c mut E: Executor<'c, Database = Postgres>
{
    let (sql, arguments) = sql_query::register_user_query(args);
    let row = sqlx::query_with(&sql, arguments).fetch_one(executor).await?;
    Ok(User::from_row(&row)?)
}

pub mod sql_query {
    use sea_query::{SelectStatement, Cond, Expr, Alias, Query, PostgresQueryBuilder};
    use sea_query_binder::{SqlxValues, SqlxBinder};

    use crate::{model::user::{UserFilter, RegisterUser}, sql::UserIden};

    pub fn filter_user(s: &mut SelectStatement, filter: UserFilter) {
        if let Some(name_or_email) = &filter.name_or_email {
            s.cond_where(
                Cond::any()
                .add(Expr::col(UserIden::Name).eq(name_or_email))
                .add(Expr::col(UserIden::Email).eq(name_or_email))
            );
        }
    }


    pub fn count_user_by_query(filter: UserFilter) -> (String, SqlxValues) {
        let mut qb = Query::select();
        
        qb
        .expr_as(Expr::col(UserIden::ID).count(), Alias::new("count"))
        .from(UserIden::Table);
        
        // Filter the user
        filter_user(&mut qb, filter); 

        qb.build_sqlx(PostgresQueryBuilder)
    }

    pub fn register_user_query(args: RegisterUser) -> (String, SqlxValues) {
        Query::insert()
            .into_table(UserIden::Table)
            .columns([
                UserIden::Name, 
                UserIden::Email, 
                UserIden::Password])
            .values([
                args.name.into(),
                args.email.into(),
                args.password.into()
            ])
            .expect("Cannot bind values")
            .returning(Query::returning().columns([
                UserIden::ID,
                UserIden::Name,
                UserIden::Email,
                UserIden::Avatar
            ])).build_sqlx(PostgresQueryBuilder)
    }
}

