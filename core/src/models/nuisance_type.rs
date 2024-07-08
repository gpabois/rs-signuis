use chrono::{DateTime, Utc};
use uuid::Uuid;

pub type NuisanceTypeId = Uuid;

pub struct CreateNuisanceType<'a> {
    pub label: &'a str,
    pub description: &'a str,
    pub family_id: Uuid,
}
pub struct InsertNuisanceType<'a> {
    pub id: Option<Uuid>,
    pub label: &'a str,
    pub description: &'a str,
    pub family_id: Uuid,
}
pub struct NuisanceType {
    pub id: Uuid,
    pub label: String,
    pub description: String,
    pub family_id: Uuid,
}
