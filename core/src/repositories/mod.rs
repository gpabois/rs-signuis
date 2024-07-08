use sqlx::PgConnection;

pub mod credential;
pub mod nuisance_family;
pub mod nuisance_report;
pub mod nuisance_type;
pub mod sessions;
pub mod users;

pub struct ChunkArgs {
    offset: Option<usize>,
    limit: Option<usize>
}

#[derive(Default, Clone)]
pub struct Repository {}
