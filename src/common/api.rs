use crate::common::file_manager::load_persisted_data;
use crate::common::shared_manager::BOSHAREDINSTANCE;
use crate::common::system_info_manager::BOSYSTEMINFOINSTANCE;
use crate::model::payload::EventType;
use crate::network::event_api::{send_event, send_personal_event, send_sdk_start};
use crate::network::manifest_api::get_manifest;

use serde_json::Value;

const MAP_ID_CODE: u64 = 21001;
const MAP_ID_NAME: &str = "map_id";
const MAP_ID_PROVIDER: &str = "map_provider";

pub async fn init(token: String, endpoint_url: String) -> bool {
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_base_url(endpoint_url.to_string());
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_token(token.to_string());

    load_persisted_data();
    BOSYSTEMINFOINSTANCE.lock().unwrap().init_system_info();

    let response = get_manifest().await;

    if response.is_ok() {
        let session_response = send_sdk_start().await;
        println!("{:?}", session_response.is_err());
        true
    } else {
        println!("Manifest pull failed. Please check sdk key and end point!");
        false
    }
}

pub async fn capture(event_name: String, event_data: String, screen_name: String) -> bool {
    let data_json: Value = serde_json::from_str(event_data.as_str()).unwrap();
    let response = send_event(event_name, EventType::Codified, screen_name, data_json, 0).await;
    response.is_ok()
}

pub async fn capture_personal(
    event_name: String,
    event_data: String,
    is_phi: bool,
    screen_name: String,
) -> bool {
    let data_json: Value = serde_json::from_str(event_data.as_str()).unwrap();
    let response = send_personal_event(event_name, screen_name, data_json, is_phi).await;
    response.is_ok()
}

pub async fn map_id(external_id: String, provider: String, data: String) -> bool {
    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    let mut data_value = data.as_object().unwrap().clone();
    data_value.insert(
        MAP_ID_NAME.to_string(),
        serde_json::Value::String(external_id),
    );
    data_value.insert(
        MAP_ID_PROVIDER.to_string(),
        serde_json::Value::String(provider),
    );

    let response = send_event(
        MAP_ID_NAME.to_string(),
        EventType::Codified,
        "".to_string(),
        Value::Object(data_value),
        MAP_ID_CODE,
    )
    .await;
    response.is_ok()
}

pub fn enable_log(enable: bool) {
    BOSHAREDINSTANCE.lock().unwrap().set_log_enabled(enable);
}

pub fn get_user_id() -> String {
    BOSHAREDINSTANCE.lock().unwrap().user_id.to_string()
}

pub fn enable_sdk(enable: bool) {
    BOSHAREDINSTANCE.lock().unwrap().set_sdk_enabled(enable);
}
