use uuid::Uuid;

/// Objet pour créer une famille de nuisance.
pub struct CreateNuisanceFamily {
    pub label: String,
    pub description: String,
}

pub type NuisanceFamilyId = Uuid;

impl From<CreateNuisanceFamily> for InsertNuisanceFamily {
    fn from(value: CreateNuisanceFamily) -> Self {
        Self {
            label: value.label,
            description: value.description,
        }
    }
}

pub struct InsertNuisanceFamily {
    pub label: String,
    pub description: String,
}

/// Une famille de nuisance (odeur, visuel, etc.)
pub struct NuisanceFamily {
    pub id: Uuid,
    pub label: String,
    pub description: String,
}