use sqlx::FromRow;

use crate::model::session::Session;

pub mod traits {
    use futures::{stream::BoxStream, future::BoxFuture};
    use sqlx::{Executor, Database};

    use crate::{model::{user::InsertSession, session::{SessionFilter, Session}}, Error, drivers};

    pub trait SessionRepository{
        fn insert_session<'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, insert: InsertSession) -> BoxFuture<'b, Result<Session, Error>>;
        fn find_session_by<'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, filter: SessionFilter) -> BoxStream<'b, Result<Session, Error>>;
    }
}

mod sql_query {
    use sea_query::Query;

    use crate::model::user::InsertSession;

    pub fn insert_session(InsertSession{ip, user_id, expires_in, token}: InsertSession) -> (String, SqlxValues) {
        let token = generate_token(16);

        let mut insert_query = Query::insert()
        .into_table(SessionIden::Table)
        .columns([
            SessionIden::UserID,
            SessionIden::IP,
            SessionIden::Token,
            SessionIden::ExpiresAt
        ]).values([
            user_id.into(),
            ip.into(),
            token.into(),
            expires_at.into()
        ])
        .expect("Cannot bind values")
        .returning(Query::returning()
            .columns([
                SessionIden::ID, 
                SessionIden::UserID, 
                SessionIden::IP
            ])
        )
        .to_owned();

        Query::with().cte(
            CommonTableExpression::new()
                .table_name(Alias::new("inserted_report"))
                .query(insert_query)
        )
        .to_owned()
        .query(
            Query::select()
                .from(Alias::new("inserted_report"))
                .left_join(UserIden::Table, Expr::col((UserIden::Table, UserIden::ID)).equals(Alias::new("id")))
                .exprs([
                    Expr::col(SessionIden::ID).as_enum(Alias::new("session_id")),
                    Expr::col(SessionIden::IP).as_enum(Alias::new("session_ip")),
                    // Left join on user
                    Expr::col(UserIden::ID).as_enum(Alias::new("user_id")),
                    Expr::col(UserIden::Name).as_enum(Alias::new("user_name")),
                    Expr::col(UserIden::Email).as_enum(Alias::new("user_email")),
                    Expr::col(UserIden::Avatar).as_enum(Alias::new("user_avatar")),
                ])
                .to_owned()
        ).build_sqlx(PostgresQueryBuilder)
    }
}


impl traits::SessionRepository for &'_ super::Repository 
{
    fn insert_session<'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, insert: InsertSession) -> BoxFuture<'b, Result<Session, Error>> 
    {
        Box::pin(async {
            let (sql, arguments) = sql_query::insert_session(insert_session);
            let row = sqlx::query_with(&sql, arguments).fetch_one(querier).await?;
            Ok(Session::from_row(&row))
        })

    }

    fn find_session_by<'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, filter: SessionFilter) -> BoxStream<'b, Result<Session, Error>> 
    {
        todo!()
    }
}