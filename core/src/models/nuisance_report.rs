
use chrono::{DateTime, Utc};
use sql_gis::{sql_types::PgGeometry, types::Point};
use uuid::Uuid;

use super::{nuisance_family::NuisanceFamily, nuisance_type::NuisanceType};

/// Identifier d'un signalemet de nuisance.
pub type NuisanceReportId = Uuid;

/// Objet pour créer un nouveau signalement de nuisance.
pub struct CreateNuisanceReport {
    pub type_id: Uuid,
    pub user_id: Option<Uuid>,
    pub location: Point,
    pub intensity: i8,
}

impl From<CreateNuisanceReport> for InsertNuisanceReport {
    fn from(value: CreateNuisanceReport) -> Self {
        Self {
            id: None,
            type_id: value.type_id,
            user_id: value.user_id,
            location: value.location,
            intensity: value.intensity
        }
    }
}

/// Objet pour insérer un signalement de nuisance.
pub struct InsertNuisanceReport {
    pub id: Option<Uuid>,
    pub type_id: Uuid,
    pub user_id: Option<Uuid>,
    pub location: PgGeometry,
    pub intensity: i8,
}

/// Objet représentant un signalement de nuisance.
pub struct NuisanceReport {
    pub id: Uuid,
    pub r#type: NuisanceReportType,
    pub user: Option<ReportUser>,
    pub location: Point,
    pub intensity: i8,
    pub created_at: DateTime<Utc>,
}

pub struct NuisanceReportType {
    pub id: Uuid,
    pub label: String,
    pub description: String,
    pub kind: NuisanceType,
}

pub struct ReportUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
}




