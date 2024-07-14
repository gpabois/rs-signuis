use chrono::{DateTime, Utc};
use sql_gis::types::Point;
use uuid::Uuid;

use super::nuisance_type::NuisanceType;

/// Identifier d'un signalemet de nuisance.
pub type NuisanceReportId = Uuid;

/// Objet pour créer un nouveau signalement de nuisance.
pub struct CreateNuisanceReport {
    pub type_id: Uuid,
    pub user_id: Option<Uuid>,
    pub location: Point,
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
