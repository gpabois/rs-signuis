use async_stream::stream;
use futures::{stream::BoxStream, future::BoxFuture, StreamExt};
use sea_query::{Cond, InsertStatement, Query, ReturningClause, Returning, SelectStatement, IntoIden, Alias, Expr, IntoCondition, PostgresQueryBuilder};
use sea_query_binder::SqlxBinder;
use sqlx::{FromRow, postgres::PgRow, Row};

use crate::{models::session::{Session, SessionFilter, InsertSession, SessionClient, SessionUser}, drivers, Error, sql::{ConditionalInsert, SessionIden, UserIden}};

pub mod traits {
    use futures::{stream::BoxStream, future::BoxFuture};

    use crate::{models::session::{SessionFilter, Session, InsertSession}, Error, drivers};

    pub trait SessionRepository<'q>{
        fn insert_session<Q: drivers::DatabaseQuerier<'q>>(self, querier: Q, insert: InsertSession) 
            -> BoxFuture<'q, Result<Session, Error>>
            where Q: 'q;
        fn find_session_by<Q: drivers::DatabaseQuerier<'q>>(self, querier: Q, filter: SessionFilter) 
            -> BoxStream<'q, Result<Session, Error>>
            where Q: 'q;
    }
}

impl Into<ConditionalInsert> for InsertSession {
    fn into(self) -> ConditionalInsert {
        ConditionalInsert::new()
        .r#if(self.id.is_some(), SessionIden::ID, || self.id.unwrap().into())
        .add(SessionIden::Token, self.token.into())
        .add(SessionIden::ClientIp, self.client.ip.into())
        .add(SessionIden::ClientUserAgent, self.client.user_agent.into())
        .r#if(self.user_id.is_some(), SessionIden::UserID, || self.user_id.unwrap().into())
        .add( SessionIden::ExpiresAt, self.expires_in.into())
        .r#if(self.created_at.is_some(), SessionIden::CreatedAt, || self.created_at.unwrap().into())
        .to_owned()
    }
}

impl InsertSession {
    pub fn into_insert_statement(self) -> InsertStatement {
        self.into()
    }
}

impl Into<InsertStatement> for InsertSession {
    fn into(self) -> InsertStatement {
        let cond: ConditionalInsert = self.into();
        let (columns, values) = cond.into_tuple();

        Query::insert()
            .into_table(SessionIden::Table)
            .columns(columns)
            .values(values)
            .unwrap()
            .to_owned()
    }
}


impl Session {
    pub fn into_select_statement_with_table<T: IntoIden>(table: T) -> SelectStatement {
        let table_iden = table.into_iden();

        Query::select()
        .from(table_iden.clone())
        .left_join(UserIden::Table, Expr::col((UserIden::Table, UserIden::ID)).equals((table_iden.clone(), SessionIden::UserID)))
        .expr_as(Expr::col((table_iden.clone(), SessionIden::ID)), Alias::new("session_id"))
        .expr_as(Expr::col((table_iden.clone(), SessionIden::Token)), Alias::new("session_token"))
        .expr_as(Expr::col((table_iden.clone(), SessionIden::ClientIp)), Alias::new("client_ip"))
        .expr_as(Expr::col((table_iden.clone(), SessionIden::ClientUserAgent)), Alias::new("client_user_agent"))
        .expr_as(Expr::col((UserIden::Table, UserIden::ID)), Alias::new("user_id"))
        .expr_as(Expr::col((UserIden::Table, UserIden::Name)), Alias::new("user_name"))
        .expr_as(Expr::col((UserIden::Table, UserIden::Email)), Alias::new("user_email"))
        .expr_as(Expr::col((UserIden::Table, UserIden::Avatar)), Alias::new("user_avatar"))
        .to_owned()
    }

    pub fn into_select_statement() -> SelectStatement {
        Self::into_select_statement_with_table(SessionIden::Table)
    }
}

impl Session {
    pub fn get_returning_clause() -> ReturningClause {
        Returning::new().columns([
            SessionIden::ID, 
            SessionIden::UserID, 
            SessionIden::ClientUserAgent,
            SessionIden::ClientIp
        ])
    }
}

impl IntoCondition for SessionFilter {
    fn into_condition(self) -> sea_query::Condition {
        let cond: Cond = self.into();
        cond
    }
}

impl Into<Cond> for SessionFilter {
    fn into(self) -> Cond {
        match self {
            Self::TokenEq(value) => Cond::any().add(Expr::col(SessionIden::Token).eq(value)),
            Self::ExpiresAtLte(value) => Cond::any().add(Expr::col(SessionIden::ExpiresAt).lte(value)),
            Self::ExpiresAtGte(value) => Cond::any().add(Expr::col(SessionIden::ExpiresAt).gte(value)),
            Self::And(children) => {
                children
                .into_iter()
                .fold(Cond::all(), |e, c| e.add::<Cond>(c.into()))            
            }
            Self::Or(children) => {
                children
                .into_iter()
                .fold(Cond::any(), |e, c| e.add::<Cond>(c.into()))
            }
        }
    }
}

impl<'r> FromRow<'r, PgRow> for SessionClient {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            ip: row.try_get("client_ip")?,
            user_agent: row.try_get("client_user_agent")?
        })
    }
}


struct OptionalUserSession(Option<SessionUser>);

impl<'r> FromRow<'r, PgRow> for OptionalUserSession
{
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let user_id = row.try_get::<Option<uuid::Uuid>, _>("user_id")?;
        Ok(match user_id {
            None => OptionalUserSession(Option::None),
            Some(id) => OptionalUserSession(Some(SessionUser {
                id: id.into(),
                name: row.get("user_name"),
                email: row.get("user_email"),
                avatar: row.get("user_avatar")
            }))
        })
    }
}

impl Into<Option<SessionUser>> for OptionalUserSession {
    fn into(self) -> Option<SessionUser> {
        self.0
    }
}

impl<'r> FromRow<'r, PgRow> for Session 
{
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Result::Ok(Session {
            id: row.try_get::<Option<uuid::Uuid>, _>("session_id")?.map(Into::into),
            client: SessionClient::from_row(row)?,
            user: OptionalUserSession::from_row(row)?.into(),
            token: row.try_get("session_token")?
        })
    }
}


mod sql_query {
    use sea_query::{Query, Alias, CommonTableExpression, PostgresQueryBuilder, InsertStatement};
    use sea_query_binder::{SqlxValues, SqlxBinder};

    use crate::models::session::{InsertSession, Session};

    pub fn insert_session(args: InsertSession) -> (String, SqlxValues) {
        //let token = generate_token(16);
        
        let insert_query: InsertStatement = args
        .into_insert_statement()
        .returning(Session::get_returning_clause())
        .to_owned();

        let cte_name = Alias::new("inserted_report");

        Query::with().cte(
            CommonTableExpression::new()
                .table_name(cte_name.clone())
                .query(insert_query)
                .to_owned()
        )
        .to_owned()
        .query(Session::into_select_statement_with_table(cte_name))
        .build_sqlx(PostgresQueryBuilder)
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

    fn find_session_by<Q: drivers::DatabaseQuerier<'q>>(self, querier: Q, filter: SessionFilter) 
        -> BoxStream<'q, Result<Session, Error>> 
        where  Q: 'q
    {
        Box::pin(stream! {
            let (sql, arguments) = Session::into_select_statement()
            .cond_where(filter)
            .build_sqlx(PostgresQueryBuilder);

            let mut row_stream = sqlx::query_with(&sql, arguments).fetch(querier);

            while let Some(row_result) = row_stream.next().await {
                yield match row_result {
                    Ok(row) => Session::from_row(&row).map_err(Error::from),
                    Err(error) => Err(Error::from(error))
                }                           
            }
        })
    }
}