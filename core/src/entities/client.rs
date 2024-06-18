use node_bindgen::{derive::node_bindgen, core::JSValue};

use crate::types::node::{JsObjectConverter, TryIntoNativeValue};


#[node_bindgen]
#[derive(Clone)]
/// Représente un client accédant au service.
pub struct Client {
    pub ip: String,
    pub user_agent: String
}

impl JSValue<'_> for Client {
    fn convert_to_rust(env: &'_ node_bindgen::core::val::JsEnv, js_value: node_bindgen::sys::napi_value) -> Result<Self, node_bindgen::core::NjError> {
        let o = JsObjectConverter::new(env.clone(), js_value);

        Ok(Self { 
            ip: o.get_property("ip")?.try_into_native_value()?, 
            user_agent: o.get_property("user_agent")?.try_into_native_value()?, 
        })
    }
}

impl Client {
    pub fn new(ip: &str, user_agent: &str) -> Self {
        Self{
            ip: ip.into(),
            user_agent: user_agent.into()
        }
    }
}