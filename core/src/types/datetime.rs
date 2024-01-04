use std::ptr;

use chrono::TimeZone;
use tracing::{error, debug, trace};
use node_bindgen::{core::{JSValue, napi_call_result, TryIntoJs, NjError}, sys::{napi_create_date, napi_get_date_value}};
use sqlx_postgres::{Postgres, PgValueRef};

pub type Utc = chrono::Utc;

#[derive(Clone)]
pub struct DateTime<Tz: TimeZone>(chrono::DateTime<Tz>);

impl<Tz: TimeZone> From<chrono::DateTime<Tz>> for DateTime<Tz> {
    fn from(value: chrono::DateTime<Tz>) -> Self {
        Self(value)
    }
}

impl<Tz: TimeZone, DB: sqlx::Database> sqlx::Type<DB> for DateTime<Tz> where uuid::Uuid: sqlx::Type<DB> {
    fn type_info() -> DB::TypeInfo {
        DateTime::<Tz>::type_info()
    }
}

impl JSValue<'_> for DateTime<Utc> {
    fn convert_to_rust(env: &'_ node_bindgen::core::val::JsEnv, js_value: node_bindgen::sys::napi_value) -> Result<Self, NjError> {
        let mut result: f64 = 0.0;
        napi_call_result!(napi_get_date_value(env.inner(), js_value, &mut result))?;
        Ok(Self::from(chrono::DateTime::<Utc>::from_timestamp(result as i64, 0).unwrap()))
    }
}

impl TryIntoJs for DateTime<Utc> {
    fn try_to_js(self, js_env: &node_bindgen::core::val::JsEnv) -> Result<node_bindgen::sys::napi_value, NjError> {
        let mut result = ptr::null_mut();
        napi_call_result!(
            node_bindgen::sys::napi_create_date(
                js_env.inner(), 
                (self.0.timestamp_millis() as f64) / 1000.0,
                &mut result
            )
        )?;
        Ok(result)
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for DateTime<chrono::Utc> {
    fn decode(value: PgValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Self(chrono::DateTime::<chrono::Utc>::decode(value)?))
    }
}

impl Into<sea_query::Value> for DateTime<Utc> {
    fn into(self) -> sea_query::Value {
        self.0.into()
    }
}