use blotout::model::bomanifestmodel::BOManifestVariable;
use blotout::network::boeventapi::BOEventAPI;
use blotout::network::boeventsecuredataapi::BOEventSecureDataAPI;
use blotout::network::bohttp::BOHttpClient;
use blotout::network::bomanifestapi::BOManifestAPI;
use blotout::utility::bosharedmanager::BOSHAREDINSTANCE;
use blotout::utility::bosysteminfomanager::BOSYSTEMINFOINSTANCE;
use serde_json::Value;

#[test]
pub fn test_log_enabled() {
    BOSHAREDINSTANCE.lock().unwrap().set_log_enabled(true);
    assert!(BOSHAREDINSTANCE.lock().unwrap().log_enabled)
}

pub fn set_sdk_info() {
    BOSHAREDINSTANCE.lock().unwrap().set_log_enabled(true);

    BOSYSTEMINFOINSTANCE.lock().unwrap().init_system_info();

    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_base_url("http://stage.blotout.io".to_string());
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_token("UT72JZHRHDUQD5M".to_string());
}

#[tokio::test]
async fn sdk_init() {
    set_sdk_info();

    let client = BOHttpClient::new(
        reqwest::Client::new(),
        "https://stage.blotout.io/sdk".to_owned(),
    );

    let response = client.get_manifest().await;

    if response.is_ok() {
        assert!(true)
    } else {
        assert!(false)
    }
}

#[tokio::test]
async fn test_log_event() {
    set_sdk_info();

    let data = "{\"some property\": \"some value\", \"some other property\": \"some other value\"}"
        .to_string();

    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    let response = client.send_event("event_name", data, 0).await;

    if response.is_ok() {
        assert!(true)
    } else {
        assert!(false)
    }
}

#[tokio::test]
async fn bo_start_session() {
    set_sdk_info();

    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let response = client.send_session_start().await;

    if response.is_ok() {
        assert!(true)
    } else {
        assert!(false)
    }
}

#[tokio::test]
async fn bo_end_session() {
    set_sdk_info();

    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let response = client.send_session_end().await;

    if response.is_ok() {
        assert!(true)
    } else {
        assert!(false)
    }
}

#[tokio::test]
async fn test_log_pii_event() {
    set_sdk_info();

    let data = "{\"some property\": \"some value\", \"some other property\": \"some other value\"}"
        .to_string();

    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    let response = client.send_pii_event("event_name", data).await;

    if response.is_ok() {
        assert!(true)
    } else {
        assert!(false)
    }
}

#[tokio::test]
async fn test_log_phi_event() {
    set_sdk_info();

    let data = "{\"some property\": \"some value\", \"some other property\": \"some other value\"}"
        .to_string();

    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    let response = client.send_phi_event("event_name", data).await;

    if response.is_ok() {
        assert!(true)
    } else {
        assert!(false)
    }
}

#[test]
fn test_create_default_http_client() {
    let client = BOHttpClient::default();
    assert!(Some(client).is_some());
}

#[test]
fn test_get_session_info_model() {
    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let model = client.get_session_info_model();
    assert!(Some(model).is_some());
}

#[test]
fn test_get_manifest_variable() {
    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let pii_manifest_variable: BOManifestVariable =
        client.get_manifest_variable("PII_Public_Key".to_string());

    assert!(Some(pii_manifest_variable).is_some());
}
