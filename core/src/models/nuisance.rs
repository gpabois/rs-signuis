use node_bindgen::{derive::node_bindgen, core::{JSValue, val::{JsObject, JsEnv}}, sys::napi_value};
use crate::types::{datetime::{DateTime, Utc}, uuid::Uuid, geojson::Geometry, node::{JsObjectConverter, TryIntoNativeValue}};

use super::Identifiable;

/// Objet pour créer une famille de nuisance.
#[node_bindgen]
pub struct CreateNuisanceFamily {
    pub label: String,
    pub description: String
}

/// Objet pour insérer une famille de nuisance dans une base de donnée.
#[node_bindgen]
pub struct InsertNuisanceFamily {
    pub id: Option<Uuid>,
    pub label: String,
    pub description: String
}

/// Une famille de nuisance (odeur, visuel, etc.)
#[node_bindgen]
pub struct NuisanceFamily {
    pub id: Uuid,
    pub label: String,
    pub description: String,
}


#[node_bindgen]
pub struct CreateNuisanceType {
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

#[node_bindgen]
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


#[node_bindgen]
#[derive(Clone)]
pub struct CreateNuisanceReport {
    pub type_id:    Uuid,
    pub user_id:    Option<Uuid>,
    pub location:   Geometry,
    pub intensity:  i8,
}

impl<'a> JSValue<'a> for CreateNuisanceReport {
    fn convert_to_rust(env: &'a JsEnv, js_value: napi_value) -> Result<Self, node_bindgen::core::NjError> {
        let obj = JsObjectConverter::from(JsObject::convert_to_rust(env, js_value)?);

        let u32_intensity: u32 = obj.get_property("intensity")?.try_into_native_value()?;

        Ok(Self {
            type_id: obj.get_property("type_id")?.try_into_native_value()?,
            user_id: obj.try_get_property("user_id")?.try_into_native_value()?,
            location: obj.get_property("location")?.try_into_native_value()?,
            intensity: u32_intensity as i8,
        })
    }
}

#[node_bindgen]
pub struct NuisanceReportFamily {
    pub id: Uuid,
    pub label: String,
    pub description: String,
}

#[node_bindgen]
pub struct NuisanceReportType {
    pub id: Uuid,
    pub label: String,
    pub description: String,
    pub family: NuisanceReportFamily
}

#[node_bindgen]
pub struct ReportUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>
}

#[node_bindgen]
pub struct NuisanceReport {
    pub id: Uuid,
    pub r#type: NuisanceReportType,
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