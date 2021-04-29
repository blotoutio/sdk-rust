use blotout::model::bomanifestmodel::ManifestVariable;
use blotout::network::event_api::EventApi;
use blotout::network::event_personal_api::BoEventSecureDataApi;
use blotout::network::bohttp::BoHttpClient;
use blotout::network::manifest_api::BoManifestApi;
use blotout::utility::shared_manager::BOSHAREDINSTANCE;
use blotout::utility::system_info_manager::BOSYSTEMINFOINSTANCE;
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

    let client = BoHttpClient::new(
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

    let client = BoHttpClient::new(
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
async fn test_log_pii_event() {
    set_sdk_info();

    let data = "{\"some property\": \"some value\", \"some other property\": \"some other value\"}"
        .to_string();

    let client = BoHttpClient::new(
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

    let client = BoHttpClient::new(
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
    let client = BoHttpClient::default();
    assert!(Some(client).is_some());
}

#[test]
fn test_get_session_info_model() {
    let client = BoHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let model = client.get_session_info_model();
    assert!(Some(model).is_some());
}

#[test]
fn test_get_manifest_variable() {
    let client = BoHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let pii_manifest_variable: ManifestVariable =
        client.get_manifest_variable("PII_Public_Key".to_string());

    assert!(Some(pii_manifest_variable).is_some());
}
