use actix::Message;
use argon2::Argon2;
use sql_builder::{bind, columns, id, insert, row_value, Symbol};
use sqlx::{prelude::Type, Decode, Encode, Executor, Postgres};

use crate::{
    error::Error,
    models::user::{UserId, UserRole},
};

use super::RepositoryOp;

impl Type<Postgres> for UserRole {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <&str as Type<Postgres>>::type_info()
    }
}

impl<'q> Encode<'q, Postgres> for UserRole {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        match self {
            UserRole::User => <&str as Encode<'q, Postgres>>::encode_by_ref(&"user", buf),
            UserRole::Administrator => <&str as Encode<'q, Postgres>>::encode_by_ref(&"admin", buf),
        }
    }
}

impl<'r> Decode<'r, Postgres> for UserRole {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(match <&'r str as Decode<'r, Postgres>>::decode(value)? {
            "admin" => UserRole::Administrator,
            _ => UserRole::User,
        })
    }
}

pub struct UserWithUsernameOrEmailExists {
    pub username: String,
    pub email: String,
}

pub struct UserWithUsernameOrEmailExistsResult {
    pub username_exists: bool,
    pub email_exists: bool,
}

impl Message for UserWithUsernameOrEmailExists {
    type Result = Result<UserWithUsernameOrEmailExistsResult, Error>;
}

const USER_WITH_USERNAME_OR_EMAIL_EXISTS_QUERY: &str = r#"
    SELECT 
        EXISTS(SELECT username FROM users WHERE username=$1) as username_exists, 
        EXISTS(SELECT email FROM users WHERE email=$2) as email_exists
"#;

impl RepositoryOp for UserWithUsernameOrEmailExists {
    type Return = UserWithUsernameOrEmailExistsResult;

    fn execute<'c, E>(
        self,
        executor: E,
    ) -> futures::prelude::future::LocalBoxFuture<'c, Result<Self::Return, Error>>
    where
        E: Executor<'c, Database = Postgres> + 'c,
    {
        Box::pin(async move {
            let (username_exists, email_exists): (bool, bool) =
                sqlx::query_as(USER_WITH_USERNAME_OR_EMAIL_EXISTS_QUERY)
                    .bind(self.username)
                    .bind(self.email)
                    .fetch_one(executor)
                    .await?;

            Ok(UserWithUsernameOrEmailExistsResult {
                username_exists,
                email_exists,
            })
        })
    }
}

pub struct InsertUser {
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub role: UserRole,
}

impl RepositoryOp for InsertUser {
    type Return = UserId;

    fn execute<'c, E>(
        mut self,
        executor: E,
    ) -> futures::prelude::future::LocalBoxFuture<'c, Result<Self::Return, Error>>
    where
        E: Executor<'c, Database = Postgres> + 'c,
    {
        Box::pin(async move {
            self.hash_password()?;

            let (sql, args) = insert(TABLE)
                .columns(columns!(
                    id!(username),
                    id!(email),
                    id!(password),
                    id!(role)
                ))
                .values(row_value!(
                    bind!(&self.username),
                    bind!(&self.email),
                    bind!(&self.password),
                    bind!(&self.role)
                ))
                .build::<sqlx::Postgres>();

            let (id,): (UserId,) = sqlx::query_as_with(&sql, args).fetch_one(executor).await?;

            Ok(id)
        })
    }
}

impl InsertUser {
    pub fn new(username: &str, email: &str) -> Self {
        Self {
            username: username.into(),
            email: email.into(),
            role: UserRole::default(),
            password: None,
        }
    }
    /// Hash the password
    pub fn hash_password(&mut self) -> Result<(), Error> {
        if let Some(pwd) = self.password.clone() {
            let salt = password_hash::SaltString::generate(rand::thread_rng());
            self.password = Some(
                password_hash::PasswordHash::generate(Argon2::default(), pwd, &salt)
                    .map_err(|_| Error::internal_error())?
                    .to_string(),
            );
        }

        Ok(())
    }
}

const TABLE: sql_builder::identifier::IdentifierRef<'_> = id!(users);

#[cfg(any(test, feature = "fixture"))]
pub mod fixtures {
    use crate::error::Error;
    use crate::models::user::{UserId, UserRole};
    use crate::repositories::{Repository, RepositoryOp};

    use super::InsertUser;

    use fake::faker::internet::fr_fr::{Password, SafeEmail, Username};

    use fake::{Dummy, Fake, Faker};
    use rand::Rng;

    #[derive(Clone)]
    pub struct InsertUserFixture {
        pub username: String,
        pub email: String,
        pub password: Option<String>,
        pub role: UserRole,
    }

    impl Default for InsertUserFixture {
        fn default() -> Self {
            Faker.fake()
        }
    }

    impl InsertUserFixture {
        pub fn new() -> Self {
            Faker.fake()
        }
    }

    impl RepositoryOp for InsertUserFixture {
        type Return = <InsertUser as RepositoryOp>::Return;

        fn execute<'c, E>(
            self,
            executor: E,
        ) -> futures::prelude::future::LocalBoxFuture<'c, Result<Self::Return, Error>>
        where
            E: sqlx::prelude::Executor<'c, Database = sqlx::Postgres> + 'c,
        {
            self.to_model().execute(executor)
        }
    }

    impl InsertUserFixture {
        pub fn to_model(self) -> InsertUser {
            InsertUser::from(self)
        }

        pub async fn execute(self, repos: &Repository) -> Result<UserId, Error> {
            let insert = self.to_model();
            repos.execute(insert).await
        }
    }

    impl From<InsertUserFixture> for InsertUser {
        fn from(
            InsertUserFixture {
                username,
                email,
                password,
                role,
            }: InsertUserFixture,
        ) -> Self {
            Self {
                username,
                email,
                password,
                role,
            }
        }
    }

    impl Dummy<Faker> for InsertUserFixture {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
            let username = Fake::fake_with_rng::<String, _>(&(Username()), rng);
            let email = Fake::fake_with_rng::<String, _>(&(SafeEmail()), rng);
            let password = Fake::fake_with_rng::<String, _>(&(Password(8..16)), rng);

            InsertUserFixture {
                username,
                email,
                password: Some(password),
                role: UserRole::default(),
            }
        }
    }
}
