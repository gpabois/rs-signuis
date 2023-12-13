use signuis_core::{services::{Database, DatabaseArgs}, config::{Config, ConfigArgs}};

#[tokio::main]
async fn main() {
    let cmd = clap::Command::new("signuis-cli")
        .bin_name("signuis-cli")
        .subcommand_required(true)
        .subcommand(clap::Command::new("db:migrate"))
        .subcommand(clap::Command::new("db:revert"));

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("db:migrate", _)) => {
            migrate_database().await;
        },
        Some(("db::revert", _)) => {
            revert_database().await;
        },
        _ => unreachable!("invalid command")
    };
}

/// Migrate the database
async fn migrate_database() {
    Config::init(ConfigArgs::Default());
    let database_url = Config::get_database_url().expect("missing 'DATABASE_URL' environment variable");
    let db = Database::new(DatabaseArgs::new(database_url.as_str())).await.unwrap();
    db.migrate().await.unwrap();
}

/// revert the database
async fn revert_database() {
    Config::init(ConfigArgs::Default());
    let database_url = Config::get_database_url().expect("missing 'DATABASE_URL' environment variable");
    let db = Database::new(DatabaseArgs::new(database_url.as_str())).await.unwrap();
    db.revert().await.unwrap();
}