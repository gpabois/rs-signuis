use futures::future::BoxFuture;
use sea_query::{IntoIden, InsertStatement, Query, IntoCondition, Cond, Expr, PostgresQueryBuilder, Func, Alias};
use sea_query_binder::SqlxBinder;
use sqlx::{FromRow, postgres::PgRow, Row};

use crate::{Error, sql::{ConditionalInsert, LogIden}, model::log::{InsertLog, Log, LogFilter, LogClient}, drivers::DatabaseQuerier};

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{model::log::{InsertLog, LogFilter, Log}, Error, drivers};

    pub trait LogsRepository<'q>: Sized + std::marker::Send +'q {
        /// Insert a new log
        fn insert_log<'b, Q: drivers::DatabaseQuerier<'b>+'b>(self, querier: Q, args: InsertLog) -> BoxFuture<'b, Result<Log,Error>>;
        /// Count the number of logs
        fn count_log_by<'b, Q: drivers::DatabaseQuerier<'b>+'b>(self, querier: Q, args: LogFilter) -> BoxFuture<'b, Result<i64,Error>> where 'b: 'q;
    }
}

impl IntoCondition for LogFilter {
    fn into_condition(self) -> sea_query::Condition {
        self.into()
    }
}

impl Into<Cond> for LogFilter {
    fn into(self) -> Cond {
        match self {
            Self::TypeEq(value) => Cond::any().add(Expr::col(LogIden::Type).eq(value)),
            Self::ClientIpEq(value) => Cond::any().add(Expr::col(LogIden::ClientIp).eq(value)),
            Self::UserIdEq(value) => Cond::any().add(Expr::col(LogIden::UserID).eq(value)),
            Self::AtGte(value) => Cond::any().add(Expr::col(LogIden::At).gte(value)),
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

impl Into<ConditionalInsert> for InsertLog {
    fn into(self) -> ConditionalInsert {
        ConditionalInsert::new()
        .add(LogIden::Type, self.r#type.into())
        .r#if_multi(self.client.is_some(), || {
            let client = self.client.unwrap();
            vec![
                (LogIden::ClientUserAgent.into_iden(), client.user_agent.into()),
                (LogIden::ClientIp.into_iden(), client.ip.into())
            ]
        })
        .r#if(self.message.is_some(), LogIden::Message, || self.message.unwrap().into())
        .r#if(self.args.is_some(), LogIden::Args, || self.args.unwrap().into())
        .r#if(self.user_id.is_some(), LogIden::UserID, || self.user_id.unwrap().into())
        .r#if(self.at.is_some(), LogIden::At, || self.at.unwrap().into())
        .to_owned()
    }
}

impl Into<InsertStatement> for InsertLog {
    fn into(self) -> InsertStatement {
        let c: ConditionalInsert = self.into();
        let (columns, values) = c.into_tuple();

        Query::insert().into_table(LogIden::Table).columns(columns).values(values).unwrap().to_owned()
    }
}

impl InsertLog {
    pub fn into_insert_statement(self) -> InsertStatement {
        self.into()
    }
}


struct OptionalLogClientDecoder;
struct LogClientDecoder;

impl LogClientDecoder {
    pub fn from_row<'q>(row: &'q PgRow) -> Result<LogClient, sqlx::Error> {
        Ok(LogClient{
            ip: row.try_get("client_ip")?,
            user_agent: row.try_get("client_user_agent")?
        })
    }
}

impl OptionalLogClientDecoder {
    pub fn from_row<'q>(row: &'q PgRow) -> Result<Option<LogClient>, sqlx::Error> {
        if row.try_get::<Option<String>,_>("client_ip")?.is_none() || row.try_get::<Option<String>,_>("client_user_agent")?.is_none() {
            return Ok(None)
        }

        Ok(Some(LogClientDecoder::from_row(row)?))
    }
}

impl<'q> FromRow<'q, PgRow> for Log {
    fn from_row(row: &'q PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            r#type: row.try_get("type")?,
            message: row.try_get("message")?,
            args: row.try_get("args")?,
            client: OptionalLogClientDecoder::from_row(row)?,
            user_id: row.try_get("user_id")?,
            at: row.try_get("at")?,
        })
    }
}

impl<'q> traits::LogsRepository<'q> for &'q super::Repository {
    fn insert_log<'b, Q: DatabaseQuerier<'b>+'b>(self, querier: Q, args: InsertLog) -> BoxFuture<'b, Result<Log, Error>> {
        Box::pin(async {
            let mut stmt = args.into_insert_statement();
            let (sql, arguments) = stmt.returning_all().build_sqlx(PostgresQueryBuilder);
            let row = sqlx::query_with(&sql, arguments).fetch_one(querier).await?;
            Ok(Log::from_row(&row)?)
        })
    }

    fn count_log_by<'b, Q: DatabaseQuerier<'b>+'b>(self, querier: Q, args: LogFilter) -> BoxFuture<'b, Result<i64, Error>> where 'b: 'q {
        let cond: Cond = args.into();
        Box::pin(async move {
            let (sql, arguments) = Query::select()
                .from(LogIden::Table)
                .expr_as(Func::count(Expr::col((LogIden::Table, LogIden::ID))), Alias::new("count"))
                .cond_where(cond)
                .build_sqlx(PostgresQueryBuilder);

            let row = sqlx::query_with(&sql, arguments).fetch_one(querier).await?;
            let count: i64 = row.try_get("count")?;
            Ok(count)
        })
    }
}