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
        let session_response = client.send_session_start().await;
        println!("{:?}", session_response.is_err());
        true
    } else {
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
    let response = client.send_event(event_name.as_str(), data).await;
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
