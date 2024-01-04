use node_bindgen::{core::{val::{JsEnv, JsObject}, NjError, JSValue}, sys::napi_value};

pub trait TryFromNApiValue<'a> {
    type Type;

    fn try_from_napi_value(js_value: napi_value, env: &'a JsEnv) -> Result<Self::Type, node_bindgen::core::NjError>;
}


impl<'a, T> TryFromNApiValue<'a> for T where T: JSValue<'a> {
    type Type = T;

    fn try_from_napi_value(js_value: napi_value, env: &'a JsEnv) -> Result<Self::Type, node_bindgen::core::NjError> {
        T::convert_to_rust(env, js_value)
    }
}


pub trait TryIntoNativeValue<T> {
    fn try_into_native_value(self) -> Result<T, node_bindgen::core::NjError>;
}

pub struct JsObjectConverter(JsObject);

impl JsObjectConverter {
    pub fn try_get_property(&self, key: &str) -> Result<Option<JsObjectConverter>, NjError> {
        Ok(self.0.get_property(key)?.map(Self::from))
    }

    pub fn get_property(&self, key: &str) -> Result<JsObjectConverter, NjError> {
        Ok(self.
            0
            .get_property(key)?
            .ok_or_else(|| NjError::Other(format!("missing property {key}").to_string()))?
            .into()
        )
    }
}

impl JsObjectConverter {
    pub fn new(env: JsEnv, napi_value: napi_value) -> Self {
        Self(JsObject::new(env, napi_value))
    }
}

impl From<JsObject> for JsObjectConverter {
    fn from(value: JsObject) -> Self {
        Self(value)
    }
}

impl<T> TryIntoNativeValue<Option<T>> for Option<JsObjectConverter> 
where for <'a> T: TryFromNApiValue<'a, Type = T>{
    fn try_into_native_value(self) -> Result<Option<T>, node_bindgen::core::NjError> {
        if let Some(o) = self {
            if o.0.napi_value() == o.0.env().get_null()? {
                return Ok(None)
            }
            return Ok(Some(T::try_from_napi_value(o.0.napi_value(), o.0.env())?))
        } else {
            return Ok(None)
        }
    }
}

impl<T> TryIntoNativeValue<T> for JsObjectConverter 
where for<'a> T: TryFromNApiValue<'a, Type = T> {
    fn try_into_native_value(self) -> Result<T, node_bindgen::core::NjError> {
        T::try_from_napi_value(self.0.napi_value(), self.0.env())
    }
}
