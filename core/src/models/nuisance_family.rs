use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Objet pour créer une famille de nuisance.
pub struct CreateNuisanceFamily {
    pub label: String,
    pub description: String,
}

pub type NuisanceFamilyId = Uuid;

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// Une famille de nuisance (odeur, visuel, etc.)
pub struct NuisanceFamily {
    pub id: Uuid,
    pub label: String,
    pub description: String,
}
