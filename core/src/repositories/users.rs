use actix::Handler;
use sqlx::Executor;
use uuid::Uuid;

use crate::{error::Error, models::user::InsertUser};

use super::Repository;

pub struct UserWithEmailOrNameExists {
    name: String,
    email: String
}

impl Handler<UserWithEmailOrNameExists> for Repository {
    type Result = Result<(bool, bool), Error>;

    async fn handle(&mut self, msg: UserWithEmailOrNameExists, ctx: &mut Self::Context) -> Self::Result {
        let conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self.pool.acquire().await?;

        sqlx::query("SELECT 
            EXISTS(SELECT username FROM users WHERE username=$1) as name_exists, 
            EXISTS(SELECT email FROM users WHERE email=$2) as email_exists
        ")
        .bind(msg.username)
        .bind(msg.email)
        .execute(conn)
        .await
    }
}

impl Repository {
    pub async fn user_with_email_or_name_exists<'c, E: Executor<'c>>(&self, executor: E, username: &str, email: &str) -> Result<(bool, bool), Error> {

    }

    pub async fn insert_user<'c, E: Executor<'c>, I: Into<InsertUser>>(&mut self, executor: E, insert: I) -> Result<Uuid, Error> {
        let insert = insert.into();
        
        sqlx::query("INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING id")
            .bind(insert.username)
            .bind(insert.email)
            .bind(insert.password)
            .execute(executor)
            .await
    }
}