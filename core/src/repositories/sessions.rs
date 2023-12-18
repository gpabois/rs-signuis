use futures::{stream::BoxStream, future::BoxFuture};
use sqlx::FromRow;

use crate::{model::{session::{Session, SessionFilter, InsertSession}}, drivers, Error};

pub mod traits {
    use futures::{stream::BoxStream, future::BoxFuture};

    use crate::{model::session::{SessionFilter, Session, InsertSession}, Error, drivers};

    pub trait SessionRepository{
        fn insert_session<'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, insert: InsertSession) -> BoxFuture<'b, Result<Session, Error>>;
        fn find_session_by<'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, filter: SessionFilter) -> BoxStream<'b, Result<Session, Error>>;
    }
}

mod sql_query {
    use sea_query::{Query, Alias, CommonTableExpression, Expr, PostgresQueryBuilder};
    use sea_query_binder::SqlxValues;

    use crate::{model::session::InsertSession, sql::{SessionIden, UserIden}};

    pub fn insert_session(InsertSession{client, user_id, expires_in, token}: InsertSession) -> (String, SqlxValues) {
        //let token = generate_token(16);

        let mut insert_query = Query::insert()
        .into_table(SessionIden::Table)
        .columns([
            SessionIden::UserID,
            SessionIden::IP,
            SessionIden::UserAgent,
            SessionIden::Token,
            SessionIden::ExpiresAt
        ]).values([
            user_id.into(),
            client.ip.into(),
            client.user_agent.into(),
            token.into(),
            expires_in.into()
        ])
        .expect("Cannot bind values")
        .returning(Query::returning()
            .columns([
                SessionIden::ID, 
                SessionIden::UserID, 
                SessionIden::UserAgent,
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
                    // Client
                    Expr::col(SessionIden::IP).as_enum(Alias::new("client_ip")),
                    Expr::col(SessionIden::UserAgent).as_enum(Alias::new("client_user_agent")),
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
    fn insert_session<'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, args: InsertSession) -> BoxFuture<'b, Result<Session, Error>> 
    {
        Box::pin(async {
            let (sql, arguments) = sql_query::insert_session(args);
            let row = sqlx::query_with(&sql, arguments).fetch_one(querier).await?;
            Ok(Session::from_row(&row))
        })

    }

    fn find_session_by<'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, filter: SessionFilter) -> BoxStream<'b, Result<Session, Error>> 
    {
        todo!()
    }
}