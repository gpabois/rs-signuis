use signuis_core::config::{Config, ConfigArgs, Mode};
use signuis_core::services::{DatabaseArgs, DatabasePool, traits::Database, DatabaseTx, ServiceTx, ServiceTxArgs};
use signuis_core::Error;
use sqlx::Postgres;

pub fn setup_config() {
    Config::init(ConfigArgs::default().set_mode(Mode::Test));
}

pub async fn setup_database() -> Result<DatabaseTx<'static, Postgres>, Error>{
    let database_url = Config::try_get_database_url()?;
    let db = DatabasePool::new(DatabaseArgs::new(database_url.as_str())).await?;
    let mut tx = db.begin().await?;
    tx.migrate().await?;
    return Result::Ok(tx);
}

pub async fn setup_services() -> Result<ServiceTx, Error>{
    let tx = setup_database().await?;
    Ok(ServiceTx::new(ServiceTxArgs::new(tx)))
}