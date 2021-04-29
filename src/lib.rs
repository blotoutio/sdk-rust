pub mod model;
pub mod network;
pub mod utility;

use crate::network::boeventapi::BoEventApi;
use crate::network::boeventsecuredataapi::BoEventSecureDataApi;
use crate::network::bohttp::BoHttpClient;
use crate::network::bomanifestapi::BoManifestApi;
use crate::utility::bosharedmanager::BOSHAREDINSTANCE;
use crate::utility::bosysteminfomanager::BOSYSTEMINFOINSTANCE;
use serde_json::Value;

const BO_EVENT_MAP_ID: u64 = 21001;
const BO_MAP_ID: &str = "map_id";
const BO_MAP_PROVIDER: &str = "map_provider";

pub async fn bo_init(token: String, endpoint_url: String) -> bool {
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_base_url(endpoint_url.to_string());
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_token(token.to_string());

    BOSYSTEMINFOINSTANCE.lock().unwrap().init_system_info();

    let client = BoHttpClient::new(reqwest::Client::new(), endpoint_url.to_owned());
    let response = client.get_manifest().await;

    if response.is_ok() {
        let session_response = client.send_sdk_start().await;
        println!("{:?}", session_response.is_err());
        true
    } else {
        println!("Manifest pull failed. Please check sdk key and end point!");
        false
    }
}

pub async fn bo_log_event(event_name: String, data: String) -> bool {
    let client = BoHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    let response = client.send_event(event_name.as_str(), data, 0).await;
    response.is_ok()
}

pub async fn bo_log_pii_event(event_name: String, data: String) -> bool {
    let client = BoHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    let response = client.send_pii_event(event_name.as_str(), data).await;
    response.is_ok()
}

pub async fn bo_log_phi_event(event_name: String, data: String) -> bool {
    let client = BoHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    let response = client.send_phi_event(event_name.as_str(), data).await;

    response.is_ok()
}

pub async fn bo_map_id(id: String, provider: String, data: String) -> bool {
    let client = BoHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    let mut map_object = data.as_object().unwrap().clone();
    map_object.insert(BO_MAP_ID.to_string(), serde_json::Value::String(id));
    map_object.insert(
        BO_MAP_PROVIDER.to_string(),
        serde_json::Value::String(provider),
    );

    let payload: Value = Value::Object(map_object);

    let response = client.send_event(BO_MAP_ID, payload, BO_EVENT_MAP_ID).await;
    response.is_ok()
}

pub fn bo_log_enabled(log_enabled: bool) {
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_log_enabled(log_enabled);
}
