use futures::{stream::BoxStream, future::BoxFuture};
use sqlx::FromRow;

use crate::{model::session::{Session, SessionFilter, InsertSession}, drivers, Error};

pub mod traits {
    use futures::{stream::BoxStream, future::BoxFuture};

    use crate::{model::session::{SessionFilter, Session, InsertSession}, Error, drivers};

    pub trait SessionRepository<'q>{
        fn insert_session<Q: drivers::DatabaseQuerier<'q>>(self, querier: Q, insert: InsertSession) 
            -> BoxFuture<'q, Result<Session, Error>>
            where Q: 'q;
        fn find_session_by<Q: drivers::DatabaseQuerier<'q>>(self, querier: Q, filter: SessionFilter) 
            -> BoxStream<'q, Result<Session, Error>>
            where Q: 'q;
    }
}

mod sql_query {
    use sea_query::{Query, Alias, CommonTableExpression, Expr, PostgresQueryBuilder};
    use sea_query_binder::{SqlxValues, SqlxBinder};

    use crate::{model::session::InsertSession, sql::{SessionIden, UserIden}};

    pub fn insert_session(InsertSession{client, user_id, expires_in, token, id}: InsertSession) -> (String, SqlxValues) {
        //let token = generate_token(16);

        let insert_query = Query::insert()
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
                .to_owned()
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

impl<'q> traits::SessionRepository<'q> for &'q super::Repository 
{
    fn insert_session<Q: drivers::DatabaseQuerier<'q>>(self, querier: Q, args: InsertSession) 
        -> BoxFuture<'q, Result<Session, Error>> 
        where Q: 'q
    {
        Box::pin(async {
            let (sql, arguments) = sql_query::insert_session(args);
            let row = sqlx::query_with(&sql, arguments).fetch_one(querier).await?;
            Session::from_row(&row).map_err(crate::Error::from)
        })

    }

    fn find_session_by<Q: drivers::DatabaseQuerier<'q>>(self, _querier: Q, _filter: SessionFilter) 
        -> BoxStream<'q, Result<Session, Error>> 
        where  Q: 'q
    {
        todo!()
    }
}