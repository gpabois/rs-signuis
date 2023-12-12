use argon2::Argon2;
use password_hash::{PasswordHash, SaltString};
use sqlx::{Postgres, Pool, Row};
use sea_query::{Query, SelectStatement, Cond, Expr, PostgresQueryBuilder, Alias};
use sea_query_binder::SqlxBinder;

use crate::{model::user::{RegisterUser, User, UserFilter}, Error, sql::UserIden};


pub struct Account {
    pool: Pool<Postgres>
}

pub fn filter_user(s: &mut SelectStatement, filter: UserFilter) {
    if let Some(name_or_email) = &filter.name_or_email {
        s.cond_where(
            Cond::any()
            .add(Expr::col(UserIden::Name).eq(name_or_email))
            .add(Expr::col(UserIden::Email).eq(name_or_email))
        );
    }
}

impl Account {
    pub async fn count_user_by(&self, filter: UserFilter) -> Result<i64, Error> {
        let mut qb = Query::select();
        
        qb
        .expr_as(Expr::col(UserIden::ID).count(), Alias::new("count"))
        .from(UserIden::Table);
        
        // Filter the user
        filter_user(&mut qb, filter);

        let (sql, values) = qb.build_sqlx(PostgresQueryBuilder);
        let result = sqlx::query_with(&sql, values).fetch_one(&self.pool).await?;
        
        Result::Ok(result.get("count"))
    }

    /// Register a new user
    pub async fn register_user(&self, mut args: RegisterUser) -> Result<User, Error> {
        args.password = PasswordHash::generate(
            Argon2::default(), 
            args.password, 
            &SaltString::generate(rand::thread_rng())
        ).expect("Cannot generate password hash").to_string();
        
        let (sql, values) = Query::insert()
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
            ])).build_sqlx(PostgresQueryBuilder);
        
        let row = sqlx::query_with(&sql, values)
        .fetch_one(&self.pool)
        .await?;

        return Result::Ok(User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            avatar: row.get("avatar")
        })
    }
}
