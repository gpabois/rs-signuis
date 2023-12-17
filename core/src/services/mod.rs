mod database;
mod account;
mod authentication;

pub use database::*;
pub use authentication::*;
pub use account::*;
use futures::future::BoxFuture;
use sqlx::{Postgres, Pool, Transaction, Acquire};

use crate::{Error, model::{session::Session, auth::Credentials}};


pub mod traits {
    pub use super::authentication::traits::Authentication;

    pub trait Hub {
        type Authentication: for <'q> Authentication<'q>;

        fn auth(self) -> Self::Authentication;
    }
}

pub struct ServicePoolArgs {
    pool: Pool<Postgres>
}

impl ServicePoolArgs {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self{pool}
    }
}

/// Service pool
pub struct ServicePool {
    pool: Pool<Postgres>
}

impl ServicePool {
    pub fn new(args: ServicePoolArgs) -> Self {
        Self{pool: args.pool}
    }

    pub async fn begin(self) -> Result<ServiceTx<'static>, Error> {
        let tx = self.pool.begin().await?;
        Ok(ServiceTx::new(tx))
    }
}

/// Service transaction
pub struct ServiceTx<'a> {
    tx: Transaction<'a, Postgres>
}

impl<'a> ServiceTx<'a> {
    pub fn new(tx: Transaction<'a, Postgres>) -> Self {
        Self {tx}
    }

    pub async fn commit(self) -> Result<(), Error> {
        self.tx.commit().await?;
        Ok(())
    }

    pub async fn rollback(self) -> Result<(), Error> {
        self.tx.rollback().await?;
        Ok(())
    }
}

impl<'q> traits::Authentication<'q> for &'q mut ServiceTx<'_> {
    fn check_token<'a, 'b>(self, token: &'a str) -> BoxFuture<'b, Result<Session, Error>> 
    where 'a: 'b, 'q: 'b
    {
        Box::pin(async {
            let conn = self.tx.acquire().await?;
            
            let session = Authentication::new(AuthenticationArgs::new(conn))
            .check_token(token)
            .await?;

            Ok(session)
        })
    }

    fn get_session_by_ip<'a, 'b>(self, ip: &'a str) -> BoxFuture<'b, Result<Session, Error>> 
    where 'a: 'b
    {
        todo!()
    }

    fn check_credentials<'a, 'b>(self, credentials: Credentials, ip: &str) -> futures::prelude::future::BoxFuture<'b, Result<crate::model::session::Session, crate::Error>> 
    where 'a: 'b
    {
        todo!()
    }
}