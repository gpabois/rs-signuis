use signuis_core::{services::{Database, DatabasePool, DatabasePoolArgs}, config::{Config, ConfigArgs}, Error};
use signuis_core::log::{info, error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cmd = clap::Command::new("signuis-cli")
        .bin_name("signuis-cli")
        .subcommand_required(true)
        .subcommand(clap::Command::new("db:migrate"))
        .subcommand(clap::Command::new("db:revert"));

    let matches = cmd.get_matches();

    let result = match matches.subcommand() {
        Some(("db:migrate", _)) => {
            migrate_database().await
        },
        Some(("db:revert", _)) => {
            revert_database().await
        },
        _ => unreachable!("invalid command")
    };

    if result.is_err() {
        error!(target: "signuis::cli", "{:?}", result.unwrap_err());
    }

    Result::Ok(())
}

/// Migrate the database
async fn migrate_database() -> Result<(), Error> {
    Config::init(ConfigArgs::default());
    let database_url = Config::try_get_database_url()?;
    info!(target: "signuis::cli", "Migrating...");
    let db = DatabasePool::new(DatabasePoolArgs::new(database_url.as_str())).await?;
    Database::migrate(&db).await?;
    info!(target: "signuis::cli", "done !");
    Result::Ok(())
}

/// Revert the database
async fn revert_database() -> Result<(), Error> {
    Config::init(ConfigArgs::default());
    let database_url = Config::try_get_database_url()?;
    info!(target: "signuis::cli", "Reverting...");
    let db = DatabasePool::new(DatabasePoolArgs::new(database_url.as_str())).await?;
    Database::revert(&db).await?;
    info!(target: "signuis::cli", "done !");
    Result::Ok(())
}