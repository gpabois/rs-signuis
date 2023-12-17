pub mod traits {
    use futures::{stream::BoxStream, future::BoxFuture};
    use sqlx::{Executor, Database};

    use crate::{model::{user::InsertSession, session::{SessionFilter, Session}}, Error};

    pub trait SessionRepository<'q> {
        type Database: Database;
        type Executor: Executor<'q>;

        fn insert<'b>(executor: Self::Executor, insert: InsertSession) -> BoxFuture<'b, Result<Session, Error>> where 'q: 'b;
        fn find_by<'b>(executor: Self::Executor, filter: SessionFilter) -> BoxStream<'b, Result<Session, Error>> where 'q: 'b;
    }
}

