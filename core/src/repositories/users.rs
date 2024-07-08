use sqlx::Executor;
use uuid::Uuid;

use crate::{error::Error, models::user::InsertUser};

use super::Repository;

impl Repository {
    pub async fn user_with_email_or_name_exists<'c, E: Executor<'c>>(&self, executor: E, username: &str, email: &str) -> Result<(bool, bool), Error> {
        sqlx::query("SELECT 
            EXISTS(SELECT username FROM users WHERE username=$1) as name_exists, 
            EXISTS(SELECT email FROM users WHERE email=$2) as email_exists
        ")
        .bind(username)
        .bind(email)
        .execute(executor)
        .await
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