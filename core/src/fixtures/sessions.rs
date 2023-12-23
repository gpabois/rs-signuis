use chrono::{DateTime, Utc};
use futures::future::BoxFuture;
use crate::{model::session::{Client, InsertSession, Session}, services::ServiceTx, repositories::sessions::traits::SessionRepository, Error};
use sqlx::Acquire;
use uuid::Uuid;

use super::{users::UserFixture, Fixture, rel::ForeignKeyFixture};

#[derive(Default)]
pub struct SessionFixture {
    id: Option<Uuid>,
    token: Option<String>,
    client: Option<Client>,
    user_id: Option<ForeignKeyFixture<Uuid, UserFixture>>,
    expires_in: Option<DateTime<Utc>>
}

impl SessionFixture {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SessionFixture {
    
    pub fn with_expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_in = Some(value);
        self
    }
    pub fn with_id<I: Into<Uuid>>(mut self, value: I) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn with_token(mut self, value: &str) -> Self {
        self.token = Some(value.into());
        self
    }
    
    pub fn with_user_id<F: Into<ForeignKeyFixture<Uuid, UserFixture>>>(mut self, value: F) -> Self {
        self.user_id = Some(value.into());
        self
    }
}

impl super::Fixture for SessionFixture {
    type Entity = Session;

    fn into_entity<'a, 'b>(mut self, tx: &'a mut ServiceTx<'_>) -> BoxFuture<'b, Result<Self::Entity, Error>> where 'a: 'b {
        Box::pin(async move {
            // Transform fixture into entity
            if let Some(user_fk) = self.user_id {
                self.user_id = Some(user_fk.into_entity(tx).await?);
            }

            let querier: &mut sqlx::PgConnection = tx.querier.acquire().await?;
            tx.repos.insert_session(querier, self.into()).await
        })
    }
}

impl Into<InsertSession> for SessionFixture {
    fn into(self) -> InsertSession {
        let mut insert = InsertSession::new(self.client.unwrap_or_else(super::clients::new_client));
        
        if let Some(id) = self.id {
            insert = insert.set_id(id);
        }

        if let Some(token) = self.token {
            insert = insert.set_token(&token);
        }

        if let Some(expires_in) = self.expires_in {
            insert = insert.set_expires_in(expires_in);
        }

        if let Some(user_fk) = self.user_id {
            insert = insert.set_user_id(user_fk.expect_id("cannot generate session insertion data with pending user fixture"));
        }

        insert
    }
}

pub async fn new_session(tx: &mut ServiceTx<'_>) -> Result<Session, Error> {
    let args = SessionFixture::new();
    new_session_with(tx, args).await
}

pub async fn new_session_with(tx: &mut ServiceTx<'_>, args: SessionFixture) -> Result<Session, Error>{
    args.into_entity(tx).await
}