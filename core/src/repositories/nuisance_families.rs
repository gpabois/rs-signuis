use sea_query::{InsertStatement, Query, ReturningClause, Returning, PostgresQueryBuilder};
use sea_query_binder::SqlxBinder;
use sqlx::{FromRow, Row, postgres::PgRow};

use crate::{
    types::uuid::Uuid,
    entities::{InsertNuisanceFamily, NuisanceFamily, Identifiable},
    sql::{ConditionalInsert, NuisanceFamilyIden}
};

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{drivers, entities::nuisance::{NuisanceFamily, InsertNuisanceFamily}, Error};

    pub trait NuisanceFamilyRepository<'q>: Sized + std::marker::Send {
        // Find credentials based on a filter
        fn insert_nuisance_family<'a, 'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, args: InsertNuisanceFamily) 
            -> BoxFuture<'b, Result<NuisanceFamily, Error>> 
        where 'a: 'b, 'q: 'b, Q: 'b;
    }
}

impl NuisanceFamily {
    pub fn get_returning_clause() -> ReturningClause {
        Returning::new().columns([
            NuisanceFamilyIden::ID,
            NuisanceFamilyIden::Label,
            NuisanceFamilyIden::Description,
        ])
    }
}

impl FromRow<'_, PgRow> for NuisanceFamily {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self { 
            id: row.try_get::<uuid::Uuid, _>("id")?.into(), 
            label: row.try_get("label")?, 
            description: row.try_get("description")?, 
        })
    }
}

impl Into<ConditionalInsert> for InsertNuisanceFamily {
    fn into(self) -> ConditionalInsert {
        ConditionalInsert::new()
        .r#if(self.id.is_some(), NuisanceFamilyIden::ID, || self.id.unwrap().into())
        .add(NuisanceFamilyIden::Label, self.label.into())
        .add(NuisanceFamilyIden::Description, self.description.into())
        .to_owned()
    }
}

impl Into<InsertStatement> for InsertNuisanceFamily {
    fn into(self) -> InsertStatement {
        let c: ConditionalInsert = self.into();
        let (columns, values) = c.into_tuple();

        Query::insert().into_table(NuisanceFamilyIden::Table).columns(columns).values(values).unwrap().to_owned()
    }
}

impl InsertNuisanceFamily {
    pub fn into_insert_statement(self) -> InsertStatement {
        self.into()
    }
}

impl<'q> traits::NuisanceFamilyRepository<'q> for &'q super::Repository 
{
    fn insert_nuisance_family<'a, 'b, Q: crate::drivers::DatabaseQuerier<'b>>(self, querier: Q, args: crate::entities::nuisance::InsertNuisanceFamily) 
        -> futures::prelude::future::BoxFuture<'b, Result<crate::entities::nuisance::NuisanceFamily, crate::Error>> 
    where 'a: 'b, 'q: 'b, Q: 'b {
        Box::pin(async {
            let (sql, arguments) = args
            .into_insert_statement()
            .returning(NuisanceFamily::get_returning_clause())
            .build_sqlx(PostgresQueryBuilder);

            let row = sqlx::query_with(&sql, arguments).fetch_one(querier).await?;

            Ok(NuisanceFamily::from_row(&row)?)
        })
    }
}

impl Identifiable for NuisanceFamily {
    type Type = Uuid;

    fn id(&self) -> Self::Type {
        self.id
    }
}