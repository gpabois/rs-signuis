use chrono::{DateTime, Utc};
use futures::future::BoxFuture;
use crate::{model::log::{Log, InsertLog, LogClient}, services::ServiceTx, repositories::logs::traits::LogsRepository, Error};
use sqlx::Acquire;
use uuid::Uuid;
use fake::{faker::lorem::fr_fr::Word, Fake};

use super::{users::UserFixture, Fixture, rel::ForeignKeyFixture};

#[derive(Default, Clone)]
pub struct LogFixture {
    id: Option<Uuid>,
    r#type: Option<String>,
    message: Option<String>,
    args: Option<String>,
    client:  Option<LogClient>,
    user_id: Option<ForeignKeyFixture<Uuid, UserFixture>>,
    at: Option<DateTime<Utc>>
}

impl LogFixture {
    pub fn new() -> Self {
        Self::default()
    }
}

impl LogFixture {
    pub fn with_id<I: Into<Uuid>>(&mut self, value: I) -> &mut Self {
        self.id = Some(value.into());
        self
    }

    pub fn with_type(&mut self, value: &str) -> &mut Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn with_message(&mut self, value: &str) -> &mut Self {
        self.message = Some(value.into());
        self
    }

    pub fn with_client(&mut self, value: LogClient) -> &mut Self {
        self.client = Some(value);
        self
    }

    pub fn with_at(&mut self, value: DateTime<Utc>) -> &mut Self {
        self.r#at = Some(value.into());
        self
    }
    
    pub fn with_user_id<F: Into<ForeignKeyFixture<Uuid, UserFixture>>>(&mut self, value: F) -> &mut Self {
        self.user_id = Some(value.into());
        self
    }
}

impl super::Fixture for LogFixture {
    type Entity = Log;

    fn into_entity<'a, 'b>(mut self, tx: &'a mut ServiceTx<'_>) -> BoxFuture<'b, Result<Self::Entity, Error>> where 'a: 'b {
        Box::pin(async move {
            // Transform fixture into entity
            if let Some(user_fk) = self.user_id {
                self.user_id = Some(user_fk.into_entity(tx).await?);
            }

            let querier: &mut sqlx::PgConnection = tx.querier.acquire().await?;
            tx.repos.insert_log(querier, self.into()).await
        })
    }
}

impl Into<InsertLog> for LogFixture {
    fn into(self) -> InsertLog {
        let mut insert = InsertLog::new(&self.r#type.unwrap_or_else(|| Word().fake()));
        
        if let Some(value) = self.id {
            insert.set_id(value);
        }
        
        if let Some(value) = self.client {
            insert.set_client(value);
        }

        if let Some(value) = self.message {
            insert.set_message(value);
        }

        if let Some(value) = self.args {
            insert.set_args(value);
        }

        if let Some(user_fk) = self.user_id {
           insert.set_user_id(user_fk.expect_id("cannot generate session insertion data with pending user fixture"));
        }

        if let Some(value) = self.at {
            insert.set_at(value);
        }

        insert
    }
}

pub async fn new_log(tx: &mut ServiceTx<'_>) -> Result<Log, Error> {
    let args = LogFixture::new();
    new_log_with(tx, args).await
}

pub async fn new_log_with(tx: &mut ServiceTx<'_>, args: LogFixture) -> Result<Log, Error>{
    args.into_entity(tx).await
}