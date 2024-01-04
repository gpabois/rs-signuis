use std::str::FromStr;

use node_bindgen::core::{TryIntoJs, JSValue, NjError};
use sea_query::SimpleExpr;
use sqlx::{Postgres, postgres::PgValueRef};

#[derive(Clone, Copy, PartialEq, std::fmt::Debug)]
pub struct Uuid(uuid::Uuid);

impl TryFrom<&'_ str> for Uuid {
    type Error = uuid::Error;

    fn try_from(value: &'_ str) -> Result<Self, Self::Error> {
        Ok(Self(uuid::Uuid::from_str(value)?))
    }
}

impl<DB: sqlx::Database> sqlx::Type<DB> for Uuid where uuid::Uuid: sqlx::Type<DB> {
    fn type_info() -> DB::TypeInfo {
        uuid::Uuid::type_info()
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for Uuid {
    fn decode(value: PgValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Self(uuid::Uuid::decode(value)?))
    }
}

impl<'a> JSValue<'a> for Uuid {
    fn convert_to_rust(env: &'a node_bindgen::core::val::JsEnv, js_value: node_bindgen::sys::napi_value) -> Result<Self, node_bindgen::core::NjError> {
        let str = String::convert_to_rust(env, js_value)?;
        Self::try_from(str.as_str())
        .map_err(|_| NjError::InvalidType("uuid".to_owned(), "str".to_owned()))
    }
}

impl TryIntoJs for Uuid {
    fn try_to_js(self, js_env: &node_bindgen::core::val::JsEnv) -> Result<node_bindgen::sys::napi_value, node_bindgen::core::NjError> {
        self.0.to_string().try_to_js(js_env)
    }
}

impl Into<SimpleExpr> for Uuid {
    fn into(self) -> SimpleExpr {
        self.0.into()
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl Into<uuid::Uuid> for Uuid {
    fn into(self) -> uuid::Uuid {
        self.0
    }
}