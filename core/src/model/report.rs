use chrono::{DateTime, Utc};
use uuid::Uuid;
use geojson::Geometry;

use super::Identifiable;

pub struct NewNuisanceFamily {
    pub label: String,
    pub description: String
}

pub struct InsertNuisanceFamily {
    pub id: Option<Uuid>,
    pub label: String,
    pub description: String
}

pub struct NuisanceFamily {
    pub id: Uuid,
    pub label: String,
    pub description: String,
}

pub struct NewNuisanceType {
    pub label: String,
    pub description: String,
    pub family_id: Uuid   
}

pub struct InsertNuisanceType {
    pub id: Option<Uuid>,
    pub label: String,
    pub description: String,
    pub family_id: Uuid  
}

pub struct NuisanceType {
    pub id: Uuid,
    pub label: String,
    pub description: String,
    pub family: NuisanceFamily
}

impl Identifiable for NuisanceType {
    type Type= Uuid;

    fn id(&self) -> Self::Type {
        self.id.clone()
    }
}


pub struct NewNuisanceReport {
    pub type_id:    Uuid,
    pub user_id:    Option<Uuid>,
    pub location:   Geometry,
    pub intensity:  i8,
}

pub struct NuisanceReportType {
    pub id: Uuid,
    pub label: String,
    pub description: String,
    pub family: NuisanceReportFamily
}

pub struct NuisanceReportFamily {
    pub id: Uuid,
    pub label: String,
    pub description: String,
}

pub struct ReportNuisanceType {
    pub id: Uuid,
    pub label: String,
    pub description: String,
    pub family: NuisanceReportFamily
}

pub struct ReportUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>
}

pub struct NuisanceReport {
    pub id: Uuid,
    pub r#type: ReportNuisanceType,
    pub user: Option<ReportUser>,
    pub location: Geometry,
    pub intensity: i8,
    pub created_at: DateTime<Utc>
}

pub struct InsertNuisanceReport {
    pub id: Option<Uuid>,
    pub type_id: Uuid,
    pub user_id: Option<Uuid>,
    pub location:   Geometry,
    pub intensity:  i8,
    pub created_at: Option<DateTime<Utc>>
}