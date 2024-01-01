use async_stream::stream;
use fake::{faker::internet::en::{Username, FreeEmail}, Fake};
use futures::{future::BoxFuture, stream::BoxStream};
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
            &self.name.unwrap_or(Username().fake_with_rng(&mut rand::thread_rng())),
            &self.email.unwrap_or(FreeEmail().fake_with_rng(&mut rand::thread_rng()))
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

pub fn new_multi<'a, 'b, 'q>(tx: &'a mut ServiceTx<'q>) -> BoxStream<'b, Result<User, Error>> 
    where 'a: 'b
{
    Box::pin(stream! {loop {
        yield UserFixture::new().into_entity(tx).await
    }})
}

pub fn new_multi_with<'q, F: Fn() -> UserFixture + Send + 'q>(tx: &'q mut ServiceTx<'q>, f: F) -> BoxStream<'q, Result<User, Error>> {
    Box::pin(stream! {loop {
        yield f().into_entity(tx).await
    }})
}