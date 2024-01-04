use geojson::Value;
use node_bindgen::core::{TryIntoJs, val::JsObject, JSValue, NjError};
use sea_query::{SimpleExpr, Expr};
use sqlx::{Postgres, postgres::PgValueRef};

use super::node::{JsObjectConverter, TryIntoNativeValue};


#[derive(Clone)]
pub struct Geometry(pub geojson::Geometry);

impl<DB: sqlx::Database> sqlx::Type<DB> for Geometry where str: sqlx::Type<DB> {
    fn type_info() -> DB::TypeInfo {
        str::type_info()
    }
}

impl JSValue<'_> for Geometry 
{
    fn convert_to_rust(env: &'_ node_bindgen::core::val::JsEnv, js_value: node_bindgen::sys::napi_value) -> Result<Self, node_bindgen::core::NjError> {
        let o = JsObjectConverter::new(env.clone(), js_value);
        
        let typ: String = o.get_property("type")?.try_into_native_value()?;

        let value = match typ.as_str() {
            "Point" => Value::Point(o.get_property("coordinates")?.try_into_native_value()?),
            "MultiPoint" => Value::MultiPoint(o.get_property("coordinates")?.try_into_native_value()?),
            "LineString" => Value::LineString(o.get_property("coordinates")?.try_into_native_value()?),
            "MultiLineString" => Value::MultiLineString(o.get_property("coordinates")?.try_into_native_value()?),
            "Polygon" => Value::Polygon(o.get_property("coordinates")?.try_into_native_value()?),
            "MultiPolygon" => Value::MultiPolygon(o.get_property("coordinates")?.try_into_native_value()?),
            "GeometryCollection" => Value::MultiPolygon(o.get_property("geometries")?.try_into_native_value()?),
            _ => return Err(NjError::InvalidType("Geometry".to_owned(), "object".to_owned()))
        };

        Ok(Self(geojson::Geometry::new(value)))
    }
}

impl Into<SimpleExpr> for Geometry {
    fn into(self) -> SimpleExpr {
        Expr::cust_with_values(
            "ST_GeomFromGeoJSON($1)", 
            [serde_json::to_string(&self).unwrap()]
        )
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for Geometry {
    fn decode(value: PgValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        let str = value.as_str()?;
        Ok(serde_json::from_str(str)?)
    }
}


impl serde::ser::Serialize for Geometry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        self.0.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for Geometry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(Self(geojson::Geometry::deserialize(deserializer)?))
    }
}

impl TryIntoJs for Geometry {
    fn try_to_js(self, js_env: &node_bindgen::core::val::JsEnv) -> Result<node_bindgen::sys::napi_value, node_bindgen::core::NjError> {
        let mut obj = JsObject::new(js_env.clone(), js_env.create_object()?);

        match self.0.value {
            geojson::Value::Point(coordinates) => {
                obj.set_property("type", js_env.create_string_utf8("Point")?)?;
                obj.set_property("coordinates", coordinates.try_to_js(js_env)?)?;
            }
            geojson::Value::MultiPoint(coordinates) => {
                obj.set_property("type", js_env.create_string_utf8("MultiPoint")?)?;
                obj.set_property("coordinates", coordinates.try_to_js(js_env)?)?;
            },
            geojson::Value::LineString(coordinates) => {
                obj.set_property("type", js_env.create_string_utf8("LineString")?)?;
                obj.set_property("coordinates", coordinates.try_to_js(js_env)?)?;
            },
            geojson::Value::MultiLineString(coordinates) => {
                obj.set_property("type", js_env.create_string_utf8("LineString")?)?;
                obj.set_property("coordinates", coordinates.try_to_js(js_env)?)?;
            },
            geojson::Value::Polygon(coordinates) => {
                obj.set_property("type", js_env.create_string_utf8("Polygon")?)?;
                obj.set_property("coordinates", coordinates.try_to_js(js_env)?)?;
            },
            geojson::Value::MultiPolygon(coordinates) => {
                obj.set_property("type", js_env.create_string_utf8("MultiPolygon")?)?;
                obj.set_property("coordinates", coordinates.try_to_js(js_env)?)?;
            },
            geojson::Value::GeometryCollection(geometries) => {
                obj.set_property("type", js_env.create_string_utf8("GeometryCollection")?)?;
                obj.set_property("geometries", geometries.into_iter().map(Geometry::from).collect::<Vec<_>>().try_to_js(js_env)?)?;                
            },
        }

        obj.try_to_js(js_env)
    }
}

impl From<geojson::Geometry> for Geometry {
    fn from(value: geojson::Geometry) -> Self {
        Self(value)
    }
}

impl Into<geojson::Geometry> for Geometry {
    fn into(self) -> geojson::Geometry{
        self.0
    }
}