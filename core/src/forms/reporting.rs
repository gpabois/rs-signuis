use serde::{Deserialize, Serialize};
use sql_gis::geojson::GeoJsonPoint;
use uuid::Uuid;

use crate::{models::nuisance_type::NuisanceTypeId, validation::Validation};

pub struct CreateNuisanceFamilyForm {
    pub label: String,
    pub description: String,
}

impl Validation for CreateNuisanceFamilyForm {
    fn assert(&self, validator: &mut crate::validation::Validator) {
        validator.assert_not_empty(
            &self.label,
            Some("le libellé ne doit pas être vide"),
            ["label"],
        )
    }
}

pub struct CreateNuisanceTypeForm {
    pub label: String,
    pub description: String,
    pub family_id: Uuid,
}

impl Validation for CreateNuisanceTypeForm {
    fn assert(&self, validator: &mut crate::validation::Validator) {
        validator.assert_not_empty(
            &self.label,
            Some("le libellé ne doit pas être vide"),
            ["label"],
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CreateNuisanceReportForm {
    pub intensity: Option<u8>,
    pub type_id: Option<NuisanceTypeId>,
    pub location: Option<GeoJsonPoint>,
}

impl Validation for CreateNuisanceReportForm {
    fn assert(&self, validator: &mut crate::validation::Validator) {
        validator.assert_is_some(
            &self.intensity,
            Some("aucune intensité n'a été définie"),
            ["intensity"],
        );

        self.intensity.inspect(|value| {
            validator.assert_in_range_inclusive(
                value,
                1..=5,
                Some("l'intensité doit être comprise entre 1 et 5"),
                ["intensity"],
            )
        });

        validator.assert_is_some(
            &self.type_id,
            Some("un type de nuisance doit être sélectionné"),
            ["type_id"],
        );

        validator.assert_is_some(
            &self.location,
            Some("une location doit être définie"),
            ["location"],
        );
    }
}
