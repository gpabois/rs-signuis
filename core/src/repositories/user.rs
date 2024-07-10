use actix::{Handler, Message, ResponseFuture};
use argon2::Argon2;
use sqlx::{prelude::FromRow, Acquire, Executor};

use crate::{error::Error, models::user::UserId};

use super::Repository;

pub struct UserWithUsernameOrEmailExists {
    pub username: String,
    pub email: String,
}

#[derive(FromRow)]
pub struct UserWithUsernameOrEmailExistsResult {
    pub username_exists: bool,
    pub email_exists: bool,
}

impl Message for UserWithUsernameOrEmailExists {
    type Result = Result<UserWithUsernameOrEmailExistsResult, Error>;
}

impl Handler<UserWithUsernameOrEmailExists> for Repository {
    type Result = Result<UserWithUsernameOrEmailExistsResult, Error>;

    fn handle(
        &mut self,
        msg: UserWithUsernameOrEmailExists,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        Box::pin(async {
            let conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self.pool.acquire().await?;

            sqlx::query(
                "SELECT 
            EXISTS(SELECT username FROM users WHERE username=$1) as username_exists, 
            EXISTS(SELECT email FROM users WHERE email=$2) as email_exists
        ",
            )
            .bind(msg.username)
            .bind(msg.email)
            .execute(conn)
            .await
        })
    }
}

pub struct InsertUser {
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub role: Option<String>,
}

impl InsertUser {
    pub fn new(username: &str, email: &str) -> Self {
        Self {
            username: username.into(),
            email: email.into(),
            role: None,
            password: None,
        }
    }

    pub fn set_role(mut self, role: &str) -> Self {
        self.role = Some(role.into());
        self
    }

    pub fn set_password(mut self, password: &str) -> Self {
        self.password = Some(password.into());
        self.hash_password()
    }

    pub fn set_hashed_password(mut self, password: &str) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Hash the password
    pub fn hash_password(&mut self) -> Result<(), Error> {
        match self.password {
            Some(pwd) => {
                let salt = password_hash::SaltString::generate(rand::thread_rng());
                self.password = Some(
                    password_hash::PasswordHash::generate(Argon2::default(), pwd, &salt)
                        .map_err(Error::internal_error)
                        .to_string()?,
                );
            }
            None => {}
        }
        Ok(())
    }
}

impl Message for InsertUser {
    type Result = Result<UserId, Error>;
}

impl Handler<InsertUser> for Repository {
    type Result = ResponseFuture<Result<UserId, Error>>;

    fn handle(&mut self, mut msg: InsertUser, ctx: &mut Self::Context) -> Self::Result {
        Box::pin(async {
            let conn = self.pool.acquire().await?;

            msg.hash_password()?;

            sqlx::query(
                "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING id",
            )
            .bind(msg.username)
            .bind(msg.email)
            .bind(msg.password)
            .execute(conn)
            .await
        })
    }
}
