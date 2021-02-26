use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BOEventModel {
    pub meta: BOMeta,
    pub events: Vec<BOEvent>,
    pub geo: BOGeo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BOEventSecureDataModel {
    pub meta: BOMeta,
    pub pii: BOSecureData,
    pub phi: BOSecureData,
    pub geo: BOGeo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BOSecureData {
    pub key: String,
    pub data: String,
    pub iv: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BOMeta {
    pub acomp: bool,
    pub plf: i64,
    pub dcomp: bool,
    pub dmft: String,
    pub appn: String,
    pub osv: String,
    pub dm: String,
    pub vpn: bool,
    pub jbrkn: bool,
    pub osn: String,
    pub appv: String,
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
pub struct BOSessionInfo {
    pub start: i64,
    pub end: i64,
    pub duration: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BOPropertiesInfo {
    #[serde(rename = "codifiedInfo")]
    pub codified_info: Value,
    pub session_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BOGeo {
    pub couc: String,
    pub city: String,
    pub conc: String,
    pub zip: i64,
    pub lat: f64,
    pub reg: String,
    pub long: f64,
}
