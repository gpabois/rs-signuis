use futures::future::BoxFuture;
use signuis_core::config::{Config, ConfigArgs, Mode};
use signuis_core::services::database::{DatabasePool, DatabasePoolArgs};
use signuis_core::services::{Service, ServicePool, ServiceTx};
use signuis_core::Error;
use sqlx::{Postgres, Pool};

pub fn setup_config() {
    Config::init(ConfigArgs::default().set_mode(Mode::Test));
}

pub async fn setup_database() -> Result<Pool<Postgres>, Error>{
    let database_url = Config::try_get_database_url()?;
    let db = DatabasePool::new(DatabasePoolArgs::new(database_url.as_str())).await?;
    Ok(db)
}

pub trait AsyncFnOnce<'a, Args, R>: std::ops::FnOnce(Args) -> BoxFuture<'a, R> {}

pub async fn with_service<F>(with: F) 
    -> Result<(), Error> 
    where for<'a, 'f, 'g> F:  std::ops::FnOnce(&'a mut ServiceTx<'g>) -> BoxFuture<'a, Result<(), Error>>
{
    setup_config();
    let pool = setup_database().await?;
    let hub: Service<Pool<Postgres>> = ServicePool::new(pool);
    let mut tx = hub.begin().await?;
    with(&mut tx).await?;
    tx.rollback().await?;
    Ok(())
}