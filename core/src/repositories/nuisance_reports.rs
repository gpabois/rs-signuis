use futures::future::BoxFuture;
use sea_query::{Expr, InsertStatement, Query, ReturningClause, Returning, IntoIden, Alias, SelectStatement};
use sqlx::{postgres::PgRow, Row};

use crate::{
    sql::{ConditionalInsert, ReportIden, UserIden, NuisanceTypeIden, NuisanceFamilyIden}, 
    entities::nuisance::{InsertNuisanceReport, NuisanceReport, NuisanceReportType, NuisanceReportFamily, ReportUser}, 
    Error, 
    drivers
};

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{drivers, entities::nuisance::{InsertNuisanceReport, NuisanceReport}, Error};

    pub trait NuisanceReportRepository<'q>: Sized + std::marker::Send {
        // Find credentials based on a filter
        fn insert_nuisance_report<'a, 'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, args: InsertNuisanceReport) 
            -> BoxFuture<'b, Result<NuisanceReport, Error>> 
        where 'a: 'b, 'q: 'b, Q: 'b;
    }
}

impl Into<ConditionalInsert> for InsertNuisanceReport {
    fn into(self) -> ConditionalInsert {
        ConditionalInsert::new()
        .r#if(self.id.is_some(), ReportIden::ID, || self.id.unwrap().into())
        .add(ReportIden::TypeId, self.type_id.into())
        .r#if(self.user_id.is_some(), ReportIden::UserId, || self.user_id.unwrap().into())
        .add(ReportIden::Location, self.location.into())
        .add(ReportIden::Intensity, self.intensity.into())
        .r#if(self.created_at.is_some(), ReportIden::CreatedAt, || self.created_at.unwrap().into())
        .to_owned()
    }
}

impl Into<InsertStatement> for InsertNuisanceReport {
    fn into(self) -> InsertStatement {
        let c: ConditionalInsert = self.into();
        let (columns, values) = c.into_tuple();

        Query::insert()
        .into_table(ReportIden::Table)
        .columns(columns)
        .values(values)
        .unwrap()
        .to_owned()
    }
}

impl InsertNuisanceReport {
    pub fn into_insert_statement(self) -> InsertStatement {
        self.into()
    }
}


struct ReportUserDecoder;

impl ReportUserDecoder {
    pub fn from_row<'r>(row: &'r PgRow) -> Result<ReportUser, Error> {
        Ok(ReportUser {  
            id: row.try_get("user_id")?,
            name: row.try_get("user_name")?,
            email: row.try_get("user_email")?,
            avatar: row.try_get("user_avatar")?
        })
    }
}

struct OptionalReportUserDecoder;
impl OptionalReportUserDecoder {
    pub fn from_row<'r>(row: &'r PgRow) -> Result<Option<ReportUser>, Error> 
    {
        let attrs = [
            row.try_get_raw("user_id")?.as_bytes().is_ok(),
            row.try_get_raw("user_name")?.as_bytes().is_ok(),
            row.try_get_raw("user_email")?.as_bytes().is_ok(),
        ];

        if attrs.into_iter().any(|r| r == false) {
            return Ok(None)
        }

        Ok(Some(ReportUserDecoder::from_row(row)?))
    } 
}
struct ReportNuisanceFamilyDecoder;

impl ReportNuisanceFamilyDecoder {
    pub fn from_row<'r>(row: &'r PgRow) -> Result<NuisanceReportFamily, Error> {
        Ok(NuisanceReportFamily {
            id: row.try_get::<uuid::Uuid, _>("nuisance_family_id")?.into(),
            label: row.try_get("nuisance_family_label")?,
            description: row.try_get("nuisance_family_description")?
        })
    } 
}
struct ReportNuisanceTypeDecoder;

impl ReportNuisanceTypeDecoder {
    pub fn from_row<'r>(row: &'r PgRow) -> Result<NuisanceReportType, Error> {
        Ok(NuisanceReportType {
            id: row.try_get("nuisance_type_id")?,
            label: row.try_get("nuisance_type_label")?,
            description: row.try_get("nuisance_type_description")?,
            family: ReportNuisanceFamilyDecoder::from_row(row)?
        })
    }
}

struct ReportDecoder;

impl ReportDecoder {
    pub fn from_row<'r>(row: &'r PgRow) -> Result<NuisanceReport, Error> {
        Ok(NuisanceReport {
            id: row.try_get("report_id")?,
            r#type: ReportNuisanceTypeDecoder::from_row(&row)?,
            user: OptionalReportUserDecoder::from_row(&row)?,
            intensity: row.try_get("report_intensity")?,
            location: row.try_get("report_location")?,
            created_at: row.try_get("report_created_at")?
        })
    }
}

impl NuisanceReport {
    pub fn get_returning_clause() -> ReturningClause {
        Returning::new().columns([
            ReportIden::ID, 
            ReportIden::TypeId, 
            ReportIden::UserId,
            ReportIden::Location,
            ReportIden::Intensity,
            ReportIden::CreatedAt
        ])
    }

    pub fn into_select_statement_with_table<T: IntoIden>(table: T) -> SelectStatement {
        let table_iden = table.into_iden();

        Query::select()
        .from(table_iden.clone())
        .left_join(UserIden::Table, Expr::col((UserIden::Table, UserIden::ID)).equals((table_iden.clone(), ReportIden::UserId)))
        .inner_join(NuisanceTypeIden::Table, Expr::col((NuisanceTypeIden::Table, NuisanceTypeIden::ID)).equals((table_iden.clone(), ReportIden::TypeId)))
        .inner_join(NuisanceFamilyIden::Table, Expr::col((NuisanceFamilyIden::Table, NuisanceFamilyIden::ID)).equals((NuisanceTypeIden::Table, NuisanceTypeIden::FamilyId)))

        .expr_as(Expr::col((table_iden.clone(), ReportIden::ID)), Alias::new("report_id"))
        .expr_as(Expr::col((table_iden.clone(), ReportIden::Intensity)), Alias::new("report_intensity"))
        .expr_as(Expr::col((table_iden.clone(), ReportIden::CreatedAt)), Alias::new("report_created_at"))
        
        .expr_as(
            Expr::cust_with_expr(
                "ST_AsGeoJSON($1)", 
                Expr::col((table_iden.clone(), ReportIden::Location))
            ), Alias::new("report_location")
        )

        .expr_as(Expr::col((NuisanceTypeIden::Table, NuisanceTypeIden::ID)), Alias::new("nuisance_type_id"))
        .expr_as(Expr::col((NuisanceTypeIden::Table, NuisanceTypeIden::Label)), Alias::new("nuisance_type_label"))
        .expr_as(Expr::col((NuisanceTypeIden::Table, NuisanceTypeIden::Description)), Alias::new("nuisance_type_description"))

        .expr_as(Expr::col((NuisanceFamilyIden::Table, NuisanceFamilyIden::ID)), Alias::new("nuisance_family_id"))
        .expr_as(Expr::col((NuisanceFamilyIden::Table, NuisanceFamilyIden::Label)), Alias::new("nuisance_family_label"))
        .expr_as(Expr::col((NuisanceFamilyIden::Table, NuisanceFamilyIden::Description)), Alias::new("nuisance_family_description"))

        .expr_as(Expr::col((UserIden::Table, UserIden::ID)), Alias::new("user_id"))
        .expr_as(Expr::col((UserIden::Table, UserIden::Name)), Alias::new("user_name"))
        .expr_as(Expr::col((UserIden::Table, UserIden::Email)), Alias::new("user_email"))
        .expr_as(Expr::col((UserIden::Table, UserIden::Avatar)), Alias::new("user_avatar"))
        .to_owned()
    }
}

mod sql_query {
    use sea_query::{Query, Alias, CommonTableExpression, PostgresQueryBuilder, InsertStatement};
    use sea_query_binder::{SqlxValues, SqlxBinder};

    use crate::entities::nuisance::{InsertNuisanceReport, NuisanceReport};

    pub fn insert_report(args: InsertNuisanceReport) -> (String, SqlxValues) {
        let insert_query: InsertStatement = args
        .into_insert_statement()
        .returning(NuisanceReport::get_returning_clause())
        .to_owned();

        let cte_name = Alias::new("inserted_report");

        Query::with().cte(
            CommonTableExpression::new()
                .table_name(cte_name.clone())
                .query(insert_query)
                .to_owned()
        )
        .to_owned()
        .query(NuisanceReport::into_select_statement_with_table(cte_name))
        .build_sqlx(PostgresQueryBuilder)
    }
}


impl<'q> traits::NuisanceReportRepository<'q> for &'q super::Repository 
{
    fn insert_nuisance_report<'a, 'b, Q: drivers::DatabaseQuerier<'b>>(self, querier: Q, args: InsertNuisanceReport) 
        -> BoxFuture<'b, Result<NuisanceReport, Error>> 
    where 'a: 'b, 'q: 'b, Q: 'b {
        Box::pin(async {
            let (sql, arguments) = sql_query::insert_report(args);
            let row = sqlx::query_with(&sql, arguments).fetch_one(querier).await?;
            Ok(ReportDecoder::from_row(&row)?)    
        })
    }
}