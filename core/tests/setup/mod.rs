use signuis_core::config::{Config, ConfigArgs, Mode};
use signuis_core::services::{ServicePool, DatabasePoolArgs, ServicePoolArgs};
use signuis_core::services::{DatabasePool};
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

pub async fn setup_services() -> Result<ServicePool, Error>{
    let tx = setup_database().await?;
    Ok(ServicePool::new(ServicePoolArgs::new(tx)))
}