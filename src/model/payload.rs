use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payload {
    pub meta: Meta,
    pub events: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalData {
    pub key: String,
    pub data: String,
    pub iv: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub sdkv: String,
    pub tz_offset: i64,
    pub user_id_created: i64,
    pub plf: i64,
    pub osn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub mid: String,
    #[serde(rename = "userid")]
    pub user_id: String,
    pub evn: String,
    pub evcs: u64,
    #[serde(rename = "type")]
    pub event_type: String,
    pub scrn: String,
    pub evt: i64,
    pub session_id: String,
    #[serde(rename = "additionalData")]
    pub additional_data: Value,
}

#[derive(strum_macros::Display)]
pub enum EventType {
    Codified,
    System,
    Pii,
    Phi,
}
