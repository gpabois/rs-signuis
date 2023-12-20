use sea_query::{Cond, Expr, Query, PostgresQueryBuilder, Alias, Func};
use sea_query_binder::SqlxBinder;
use sqlx::Row;

use crate::{model::user::{InsertUser, User, UserFilter}, sql::UserIden};

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{model::user::{InsertUser, User, UserFilter}, Error, drivers};

    pub trait UserRepository<'q>: Sized + std::marker::Send +'q {
        /// Insert a new user
        fn insert_user<'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, args: InsertUser) -> BoxFuture<'b, Result<User,Error>>;
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

impl<'q> traits::UserRepository<'q> for &'q super::Repository {
    fn insert_user<'b, Q: crate::drivers::DatabaseQuerier<'b>>(self, _querier: Q, _args: InsertUser) -> futures::prelude::future::BoxFuture<'b, Result<User,crate::Error>> {
        todo!()
    }

    fn count_user_by<'b, Q: crate::drivers::DatabaseQuerier<'b>+'b>(self, querier: Q, args: UserFilter) -> futures::prelude::future::BoxFuture<'b, Result<i64,crate::Error>> where 'b :'q
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