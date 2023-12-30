use futures::future::BoxFuture;
use sea_query::{InsertStatement, Query, ReturningClause, Returning, IntoIden, SelectStatement, Expr, Alias};
use sqlx::{FromRow, Row, postgres::PgRow};

use crate::{sql::{ConditionalInsert, NuisanceFamilyIden, NuisanceTypeIden}, model::report::{ NuisanceFamily, InsertNuisanceType, NuisanceType}, drivers};

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{drivers, model::report::{InsertNuisanceType, NuisanceType}, Error};

    pub trait NuisanceTypeRepository<'q>: Sized + std::marker::Send {
        // Find credentials based on a filter
        fn insert_nuisance_type<'a, 'b, Q: drivers::DatabaseQuerier<'b>, I: Into<InsertNuisanceType> + std::marker::Send + 'b>(self, querier: Q, args: I) 
            -> BoxFuture<'b, Result<NuisanceType, Error>> 
        where 'a: 'b, 'q: 'b, Q: 'b;
    }
}

impl NuisanceType {
    pub fn get_returning_clause() -> ReturningClause {
        Returning::new().columns([
            NuisanceTypeIden::ID,
            NuisanceTypeIden::Label,
            NuisanceTypeIden::Description,
            NuisanceTypeIden::FamilyId
        ])
    }

    pub fn into_select_statement_with_table<T: IntoIden>(table: T) -> SelectStatement {
        let table_iden = table.into_iden();

        Query::select()
        .from(table_iden.clone())
        .inner_join(NuisanceFamilyIden::Table, Expr::col((NuisanceFamilyIden::Table, NuisanceFamilyIden::ID)).equals((table_iden.clone(), NuisanceTypeIden::FamilyId)))
        .expr_as(Expr::col((table_iden.clone(), NuisanceTypeIden::ID)), Alias::new("nuisance_type_id"))
        .expr_as(Expr::col((table_iden.clone(), NuisanceTypeIden::Label)), Alias::new("nuisance_type_label"))
        .expr_as(Expr::col((table_iden.clone(), NuisanceTypeIden::Description)), Alias::new("nuisance_type_description"))
        .expr_as(Expr::col((NuisanceFamilyIden::Table, NuisanceFamilyIden::ID)), Alias::new("nuisance_family_id"))
        .expr_as(Expr::col((NuisanceFamilyIden::Table, NuisanceFamilyIden::Label)), Alias::new("nuisance_family_label"))
        .expr_as(Expr::col((NuisanceFamilyIden::Table, NuisanceFamilyIden::Description)), Alias::new("nuisance_family_description"))
        .to_owned()
    }
}

struct NuisanceFamilyDecoder;

impl NuisanceFamilyDecoder {
    pub fn from_row(row: &PgRow) -> Result<NuisanceFamily, sqlx::Error> {
        Ok(NuisanceFamily{
            id: row.try_get("nuisance_family_id")?,
            label: row.try_get("nuisance_family_label")?,
            description: row.try_get("nuisance_family_description")?,
        })
    }
}

impl<'r> FromRow<'r, PgRow> for NuisanceType {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self { 
            id: row.try_get("nuisance_type_id")?, 
            label: row.try_get("nuisance_type_label")?, 
            description: row.try_get("nuisance_type_description")?,
            family: NuisanceFamilyDecoder::from_row(row)?
        })
    }
}

impl Into<ConditionalInsert> for InsertNuisanceType {
    fn into(self) -> ConditionalInsert {
        ConditionalInsert::new()
        .r#if(self.id.is_some(), NuisanceTypeIden::ID, || self.id.unwrap().into())
        .add(NuisanceTypeIden::Label, self.label.into())
        .add(NuisanceTypeIden::Description, self.description.into())
        .add(NuisanceTypeIden::FamilyId, self.family_id.into())
        .to_owned()
    }
}

impl Into<InsertStatement> for InsertNuisanceType {
    fn into(self) -> InsertStatement {
        let c: ConditionalInsert = self.into();
        let (columns, values) = c.into_tuple();

        Query::insert()
        .into_table(NuisanceTypeIden::Table)
        .columns(columns)
        .values(values)
        .unwrap()
        .to_owned()
    }
}

impl InsertNuisanceType {
    pub fn into_insert_statement(self) -> InsertStatement {
        self.into()
    }
}

mod sql_query {
    use sea_query::{Query, Alias, CommonTableExpression, PostgresQueryBuilder, InsertStatement};
    use sea_query_binder::{SqlxValues, SqlxBinder};

    use crate::model::report::{InsertNuisanceType, NuisanceType};

    pub fn insert_nuisance_type(args: InsertNuisanceType) -> (String, SqlxValues) {
        let insert_query: InsertStatement = args
        .into_insert_statement()
        .returning(NuisanceType::get_returning_clause())
        .to_owned();

        let cte_name = Alias::new("inserted_nuisance_type");

        Query::with().cte(
            CommonTableExpression::new()
                .table_name(cte_name.clone())
                .query(insert_query)
                .to_owned()
        )
        .to_owned()
        .query(NuisanceType::into_select_statement_with_table(cte_name))
        .build_sqlx(PostgresQueryBuilder)
    }
}

impl<'q> traits::NuisanceTypeRepository<'q> for &'q super::Repository 
{
    fn insert_nuisance_type<'a, 'b, Q: drivers::DatabaseQuerier<'b>, I: Into<InsertNuisanceType> + std::marker::Send + 'b>(self, querier: Q, args: I) 
        -> BoxFuture<'b, Result<NuisanceType, crate::Error>> 
    where 'a: 'b, 'q: 'b, Q: 'b {
        Box::pin(async {
            let insert = args.into();
            let (sql, arguments) = sql_query::insert_nuisance_type(insert);
            let row = sqlx::query_with(&sql, arguments).fetch_one(querier).await?;
            Ok(NuisanceType::from_row(&row)?)
        })
    }
}

