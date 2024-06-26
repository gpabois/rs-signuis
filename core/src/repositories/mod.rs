use sqlx::PgConnection;

pub mod credential;
pub mod nuisance_family;
pub mod nuisance_reports;
pub mod nuisance_types;
pub mod sessions;
pub mod users;

pub struct ChunkArgs {
    offset: Option<usize>,
    limit: Option<usize>
}

#[derive(Default, Clone)]
pub struct Repository {
    conn: PgConnection,
}
