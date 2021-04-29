use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoEventModel {
    pub meta: BoMeta,
    pub events: Vec<BoEvent>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoEventSecureDataModel {
    pub meta: BoMeta,
    pub pii: BoSecureData,
    pub phi: BoSecureData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoSecureData {
    pub key: String,
    pub data: String,
    pub iv: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoMeta {
    pub plf: i64,
    pub osn: String,
    pub sdkv: String,
    pub tz_offset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoEvent {
    pub appb: i64,
    pub mid: String,
    pub id: String,
    pub evn: String,
    pub count: i64,
    pub evcs: u64,
    pub uustate: Vec<i64>,
    pub userid: String,
    pub value: String,
    pub scrn: String,
    pub evt: i64,
    pub properties: BoPropertiesInfo,
    pub nvg_tm: Vec<f64>,
    pub tst: i64,
    pub nvg: Vec<String>,
    pub nmo: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoPropertiesInfo {
    #[serde(rename = "codifiedInfo")]
    pub codified_info: Value,
    pub session_id: String,
}
