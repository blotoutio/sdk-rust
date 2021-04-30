use crate::common::crypto::encrypt_data;
use crate::common::shared_manager::BOSHAREDINSTANCE;
use crate::common::utils::{code_for_codified_event, get_mid};
use crate::model::manifest::ManifestVariable;
use crate::model::payload::{Event, EventType, PersonalData};
use crate::network::http::{Http, HttpClient};
use crate::network::manifest_api::get_manifest_variable;

use chrono::Utc;
use failure::Error;
use serde_json::{json, Value};

const SDK_START_CODE: u64 = 11130;
const SDK_START_NAME: &str = "sdk_start";

pub async fn send_event(
    event_name: String,
    event_type: EventType,
    screen_name: String,
    event_data: Value,
    event_code: u64,
) -> Result<(), Error> {
    if !BOSHAREDINSTANCE.lock().unwrap().sdk_enabled {
        return Ok(());
    }

    let mut event_sub_code = event_code;

    if event_code == 0 {
        event_sub_code = code_for_codified_event(event_name.to_string());
    }

    let session_id = BOSHAREDINSTANCE.lock().unwrap().session_id.to_string();

    let events: Vec<Event> = vec![Event {
        mid: get_mid(event_name.to_string()),
        user_id: BOSHAREDINSTANCE.lock().unwrap().user_id.to_string(),
        evn: event_name.to_string(),
        evcs: event_sub_code,
        event_type: event_type.to_string().to_lowercase(),
        scrn: screen_name,
        evt: Utc::now().timestamp_millis(),
        session_id,
        additional_data: event_data,
    }];

    let client = HttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    client.post_events(events).await
}

pub async fn send_sdk_start() -> Result<(), Error> {
    let data = json!({});
    send_event(
        SDK_START_NAME.to_string(),
        EventType::System,
        "".to_string(),
        data,
        SDK_START_CODE,
    )
    .await
}

pub async fn send_personal_event(
    event_name: String,
    screen_name: String,
    event_data: Value,
    is_phi: bool,
) -> Result<(), Error> {
    let public_key: ManifestVariable;
    let event_type: EventType;
    if is_phi {
        public_key = get_manifest_variable("PHI_Public_Key".to_string());
        event_type = EventType::Phi;
    } else {
        public_key = get_manifest_variable("PII_Public_Key".to_string());
        event_type = EventType::Pii;
    }

    let data: PersonalData = encrypt_data(event_data, public_key.value);

    send_event(event_name, event_type, screen_name, json!(data), 0).await
}
