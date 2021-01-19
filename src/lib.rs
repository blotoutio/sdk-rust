pub mod model;
pub mod network;
pub mod utility;

use crate::network::boeventapi::BOEventAPI;
use crate::network::boeventsecuredataapi::BOEventSecureDataAPI;
use crate::network::bohttp::BOHttpClient;
use crate::network::bomanifestapi::BOManifestAPI;
use crate::utility::bosharedmanager::BOSHAREDINSTANCE;
use crate::utility::bosysteminfomanager::BOSYSTEMINFOINSTANCE;
use serde_json::Value;

const BO_EVENT_MAP_ID: u64 = 21001;
const BO_MAP_ID: &str = "map_id";
const BO_MAP_PROVIDER: &str = "map_provider";

pub fn bo_log_enabled(log_enabled: bool) {
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_log_enabled(log_enabled);
}

pub async fn bo_sdk_init(token: String, end_point: String, bundle_id: String) -> bool {
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_base_url(end_point.to_string());
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_bundle_id(bundle_id.to_string());
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_token(token.to_string());

    BOSYSTEMINFOINSTANCE.lock().unwrap().init_system_info();

    let client = BOHttpClient::new(reqwest::Client::new(), end_point.to_owned());

    let response = client.get_manifest().await;

    if response.is_ok() {
        println!("SDK Intialized");
        let session_response = client.send_session_start().await;
        println!("{:?}", session_response.is_err());
        true
    } else {
        println!("SDK Intialization Error, Please check sdk key and end point !");
        false
    }
}

pub async fn bo_start_session() -> bool {
    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );
    let response = client.send_session_start().await;
    response.is_ok()
}

pub async fn bo_end_session() -> bool {
    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let response = client.send_session_end().await;
    response.is_ok()
}

pub async fn bo_log_event(event_name: String, data: String) -> bool {
    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    let response = client.send_event(event_name.as_str(), data, 0).await;
    response.is_ok()
}

pub async fn bo_log_pii_event(event_name: String, data: String) -> bool {
    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    let response = client.send_pii_event(event_name.as_str(), data).await;
    response.is_ok()
}

pub async fn bo_log_phi_event(event_name: String, data: String) -> bool {
    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    let response = client.send_phi_event(event_name.as_str(), data).await;

    response.is_ok()
}

pub async fn bo_map_id(id: String, provider: String, data: String) -> bool {
    let client = BOHttpClient::new(
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
