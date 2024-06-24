use chrono::{DateTime, Utc};
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use postgis_diesel::sql_types::Geometry;
use uuid::Uuid;

/// Objet pour créer une famille de nuisance.
#[derive(Insertable)]
#[diesel(table_name = crate::schema::nuisance_families)]
pub struct NewNuisanceFamily<'a> {
    pub label: &'a str,
    pub description: &'a str,
}

impl From<NewNuisanceFamily<'_>> for InsertNuisanceFamily<'_> {
    fn from(value: NewNuisanceFamily<'_>) -> Self {
        Self {
            id: None,
            label: value.label,
            description: value.description,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::nuisance_families)]
pub struct InsertNuisanceFamily<'a> {
    pub id: Option<Uuid>,
    pub label: &'a str,
    pub description: &'a str,
}

/// Une famille de nuisance (odeur, visuel, etc.)
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::nuisance_families)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NuisanceFamily {
    pub id: Uuid,
    pub label: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::nuisance_types)]
pub struct NewNuisanceType<'a> {
    pub label: &'a str,
    pub description: &'a str,
    pub family_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::nuisance_types)]
pub struct InsertNuisanceType<'a> {
    pub id: Option<Uuid>,
    pub label: &'a str,
    pub description: &'a str,
    pub family_id: Uuid,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::nuisance_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NuisanceType {
    pub id: Uuid,
    pub label: String,
    pub description: String,
    pub family_id: Uuid,
}

#[derive(Clone)]
pub struct NewNuisanceReport {
    pub type_id: Uuid,
    pub user_id: Option<Uuid>,
    pub location: Geometry,
    pub intensity: i8,
}

pub struct NuisanceReportFamily {
    pub id: Uuid,
    pub label: String,
    pub description: String,
}

pub struct NuisanceReportType {
    pub id: Uuid,
    pub label: String,
    pub description: String,
    pub family: NuisanceReportFamily,
}

pub struct ReportUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
}

pub struct NuisanceReport {
    pub id: Uuid,
    pub r#type: NuisanceReportType,
    pub user: Option<ReportUser>,
    pub location: Geometry,
    pub intensity: i8,
    pub created_at: DateTime<Utc>,
}
