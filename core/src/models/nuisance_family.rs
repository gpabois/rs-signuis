use chrono::{DateTime, Utc};
use diesel::{expression::AsExpression, prelude::*};
use uuid::Uuid;

/// Objet pour créer une famille de nuisance.
#[derive(Insertable)]
#[diesel(table_name = crate::schema::nuisance_families)]
pub struct NewNuisanceFamily<'a> {
    pub label: &'a str,
    pub description: &'a str,
}

pub type NuisanceFamilyId = Uuid;

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