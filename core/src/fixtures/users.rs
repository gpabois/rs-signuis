use fake::{faker::internet::fr_fr::{Username, SafeEmail}, Fake};
use futures::future::BoxFuture;
use crate::{Error, model::user::{InsertUser, User}, services::ServiceTx, repositories::users::traits::UserRepository};
use sqlx::Acquire;
use uuid::Uuid;

use super::Fixture;
#[derive(Clone)]
pub struct UserFixture {
    id: Option<Uuid>,
    name: Option<String>,
    email: Option<String>,
    password: Option<String>
}

impl UserFixture {
    pub fn new() -> Self {
        Self{id: None, name: None, email: None, password: None}
    }

    pub fn with_id(mut self, value: &str) -> Self {
        self.id = Some(Uuid::parse_str(value).unwrap());
        self        
    }

    pub fn with_name(mut self, value: &str) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn with_email(mut self, value: &str) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn with_password(mut self, value: &str) -> Self {
        self.password = Some(value.into());
        self
    }
}

impl super::Fixture for UserFixture {
    type Entity = User;

    fn into_entity<'a, 'b>(self, tx: &'a mut ServiceTx<'_>) -> BoxFuture<'b, Result<User, Error>> where 'a: 'b {
        Box::pin(async move {
            let querier: &mut sqlx::PgConnection = tx.querier.acquire().await?;
            tx.repos.insert_user(querier, self.into()).await
        })
    }
}

impl Into<InsertUser> for UserFixture {
    fn into(self) -> InsertUser {
        let mut args = InsertUser::new(
            &self.name.unwrap_or(Username().fake()),
            &self.email.unwrap_or(SafeEmail().fake())
        );

        if let Some(pwd) = self.password {
            args = args.set_password(&pwd);
        }

        if let Some(id) = self.id {
            args = args.set_id(id);
        }

        args
    }
}

pub async fn new_user<'q>(tx: &mut ServiceTx<'q>) -> Result<User, Error> {
    let args = UserFixture::new();
    new_user_with(tx, args).await
}

pub async fn new_user_with<'q>(tx: &mut ServiceTx<'q>, args: UserFixture) -> Result<User, Error> {
    args.into_entity(tx).await
}