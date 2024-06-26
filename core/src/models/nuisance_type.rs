use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;
use crate::schema;

pub type NuisanceTypeId = Uuid;

#[derive(Insertable)]
#[diesel(table_name = schema::nuisance_types)]
pub struct NewNuisanceType<'a> {
    pub label: &'a str,
    pub description: &'a str,
    pub family_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = schema::nuisance_types)]
pub struct InsertNuisanceType<'a> {
    pub id: Option<Uuid>,
    pub label: &'a str,
    pub description: &'a str,
    pub family_id: Uuid,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::nuisance_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NuisanceType {
    pub id: Uuid,
    pub label: String,
    pub description: String,
    pub family_id: Uuid,
}
