use sqlx::PgConnection;

pub mod credentials;
pub mod nuisance_families;
pub mod nuisance_reports;
pub mod nuisance_types;
pub mod sessions;
pub mod users;

#[derive(Default, Clone)]
pub struct Repository {
    conn: PgConnection,
}
