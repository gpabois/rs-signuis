use signuis_core::{services::{ServicePool, database::{DatabasePool, DatabasePoolArgs}}, config::Config, Error, fixtures::{self, nuisance_types::NuisanceTypeFixture, rel::ForeignKeyFixture, nuisance_reports::NuisanceReportFixture}};
use signuis_core::log::{info, error};
use futures::stream::StreamExt;
use rand::seq::SliceRandom;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cmd = clap::Command::new("signuis-cli")
        .bin_name("signuis-cli")
        .subcommand_required(true)
        .subcommand(clap::Command::new("db:migrate"))
        .subcommand(clap::Command::new("db:revert"))
        .subcommand(clap::Command::new("dev:reset"))
        .subcommand(clap::Command::new("dev:gen:fixtures"));

    let matches = cmd.get_matches();

    let result = match matches.subcommand() {
        Some(("db:migrate", _)) => {
            migrate_database().await
        },
        Some(("db:revert", _)) => {
            revert_database().await
        },
        Some(("dev:reset", _)) => {
            reset().await
        },
        Some(("dev:gen:fixtures", _)) => {
            generate_fixtures().await
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
    Config::init()?;
    let database_url = Config::try_get_database_url()?;
    info!(target: "signuis::cli", "Migrating...");
    let db = DatabasePool::new(DatabasePoolArgs::new(database_url.as_str())).await?;
    let hub = ServicePool::new(db);
    hub.migrate_database().await?;
    info!(target: "signuis::cli", "done !");
    Result::Ok(())
}

/// Revert the database
async fn revert_database() -> Result<(), Error> {
    Config::init()?;
    let database_url = Config::try_get_database_url()?;
    info!(target: "signuis::cli", "Reverting...");
    let db = DatabasePool::new(DatabasePoolArgs::new(database_url.as_str())).await?;
    let hub = ServicePool::new(db);
    hub.revert_database().await?;
    info!(target: "signuis::cli", "done !");
    Result::Ok(())
}

async fn reset() -> Result<(), Error> {
    revert_database().await?;
    migrate_database().await?;
    generate_fixtures().await
}

async fn generate_fixtures() -> Result<(), Error> {
    Config::init()?;
    let pool = ServicePool::from_config().await?;
    info!(target: "signuis::cli", "Generating fixtures...");
    pool.with::<(), Error, _>(|tx| {
        Box::pin(async {
            let users = fixtures::users::new_multi(tx)
            .map(Result::unwrap)
            .take(100)
            .collect::<Vec<_>>()
            .await;

            let families = fixtures::nuisance_families::new_multi(tx)
            .map(Result::unwrap)
            .take(10)
            .collect::<Vec<_>>()
            .await;

            let types = fixtures::nuisance_types::new_multi_with(tx, || {
                let family = families.choose(&mut rand::thread_rng()).unwrap();
                NuisanceTypeFixture::new()
                .with_family(ForeignKeyFixture::ID(family.id))
                .to_owned()
            }).map(Result::unwrap).take(100).collect::<Vec<_>>().await;
            
            fixtures::nuisance_reports::new_multi_with(tx, || {
                let typ = types.choose(&mut rand::thread_rng()).unwrap();
                let user = users.choose(&mut rand::thread_rng()).unwrap();

                NuisanceReportFixture::new()
                .with_type(ForeignKeyFixture::ID(typ.id))
                .with_user(ForeignKeyFixture::ID(user.id))
                .to_owned()
            }).map(Result::unwrap).take(1_000_000).collect::<Vec<_>>().await;

            Ok(())
        })
    }).await?;
    
    info!(target: "signuis::cli", "done !");

    Ok(())
}