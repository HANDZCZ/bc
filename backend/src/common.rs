use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub error: String,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct JsonString(pub Box<RawValue>);

impl From<String> for JsonString {
    fn from(value: String) -> Self {
        JsonString(RawValue::from_string(value).unwrap())
    }
}
