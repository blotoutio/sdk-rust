use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BOEventModel {
    pub meta: BOMeta,
    pub events: Vec<BOEvent>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BOEventSecureDataModel {
    pub meta: BOMeta,
    pub pii: BOSecureData,
    pub phi: BOSecureData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BOSecureData {
    pub key: String,
    pub data: String,
    pub iv: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BOMeta {
    pub plf: i64,
    pub osn: String,
    pub sdkv: String,
    pub tz_offset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BOEvent {
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
    pub properties: BOPropertiesInfo,
    pub nvg_tm: Vec<f64>,
    pub tst: i64,
    pub nvg: Vec<String>,
    pub nmo: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BOPropertiesInfo {
    #[serde(rename = "codifiedInfo")]
    pub codified_info: Value,
    pub session_id: String,
}
