use futures::future::BoxFuture;
use sea_query::{Cond, Expr, Query, PostgresQueryBuilder, Alias, Func, InsertStatement, ReturningClause, Returning};
use sea_query_binder::SqlxBinder;
use sqlx::{Row, FromRow, postgres::PgRow};

use crate::{model::user::{InsertUser, User, UserFilter}, sql::{UserIden, ConditionalInsert}, Error};

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{model::user::{InsertUser, User, UserFilter}, Error, drivers};

    pub trait UserRepository<'q>: Sized + std::marker::Send +'q {
        /// Insert a new user
        fn insert_user<'b, Q: drivers::DatabaseQuerier<'b>+'b>(self, querier: Q, args: InsertUser) -> BoxFuture<'b, Result<User,Error>>;
        /// Count the number of user
        fn count_user_by<'b, Q: drivers::DatabaseQuerier<'b>+'b>(self, querier: Q, args: UserFilter) -> BoxFuture<'b, Result<i64,Error>> where 'b: 'q;
        /// Check if at least one user match the fiter
        fn any_users_by<'b: 'q, Q: drivers::DatabaseQuerier<'b> +'b>(self, querier: Q, args: UserFilter) -> BoxFuture<'b, Result<bool,Error>> 
        where Self: 'b
        {
            Box::pin(async {
                let count = self.count_user_by(querier, args).await?;
                Ok(count > 0)
            })
        }
    }
}

impl Into<Cond> for UserFilter {
    fn into(self) -> Cond {
        match self {
            Self::Name(value) => Cond::any().add(Expr::col(UserIden::Name).eq(value)),
            Self::Email(value) => Cond::any().add(Expr::col(UserIden::Email).eq(value)),
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

impl Into<ConditionalInsert> for InsertUser {
    fn into(self) -> ConditionalInsert {
        let mut cond = ConditionalInsert::new();

        cond
            .r#if(self.id.is_some(), UserIden::ID, || self.id.unwrap().into())
            .add(UserIden::Name, self.name.into())
            .add(UserIden::Email, self.email.into())
            .r#if(self.password.is_some(), UserIden::Password, || self.password.unwrap().into());

        cond
    }
}

impl Into<InsertStatement> for InsertUser {
    fn into(self) -> InsertStatement {
        let cond: ConditionalInsert = self.into();
        let (columns, values) = cond.into_tuple();
        Query::insert().into_table(UserIden::Table).columns(columns).values(values).unwrap().to_owned()
    }
}

impl User {
    pub fn get_returning_clause() -> ReturningClause {
        Returning::new().columns([
            UserIden::ID,
            UserIden::Name,
            UserIden::Email,
            UserIden::Avatar
        ])
    }
}

impl<'r> FromRow<'r, PgRow> for User {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            email: row.try_get("email")?,
            avatar: row.try_get("avatar")?
        })
    }
}

impl<'q> traits::UserRepository<'q> for &'q super::Repository {
    fn insert_user<'b, Q: crate::drivers::DatabaseQuerier<'b>+'b>(self, querier: Q, args: InsertUser) -> BoxFuture<'b, Result<User, Error>> {
        Box::pin(async {
            let mut stmt: InsertStatement = args.into();
            stmt.returning(User::get_returning_clause());

            let (sql, arguments) = stmt.build_sqlx(PostgresQueryBuilder);
            let row = sqlx::query_with(&sql, arguments).fetch_one(querier).await?;
            Ok(User::from_row(&row)?)
        })

    }

    fn count_user_by<'b, Q: crate::drivers::DatabaseQuerier<'b>+'b>(self, querier: Q, args: UserFilter) -> BoxFuture<'b, Result<i64, Error>> where 'b :'q
    {
        let cond: Cond = args.into();
        Box::pin(async move {

            let (sql, arguments) = Query::select()
                .expr_as(Func::count(Expr::col(UserIden::ID)), Alias::new("count"))
                .cond_where(cond)
                .build_sqlx(PostgresQueryBuilder);

            let row = sqlx::query_with(&sql, arguments).fetch_one(querier).await?;
            let count: i64 = row.try_get("count")?;
            Ok(count)
        })
    }
}