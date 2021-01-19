use blotout::network::boeventapi::BOEventAPI;
use blotout::network::boeventsecuredataapi::BOEventSecureDataAPI;
use blotout::network::bohttp::BOHttpClient;
use blotout::network::bomanifestapi::BOManifestAPI;
use blotout::utility::bosharedmanager::BOSHAREDINSTANCE;
use blotout::utility::bosysteminfomanager::BOSYSTEMINFOINSTANCE;
use serde_json::Value;

#[tokio::main]
async fn main() {
    bo_log_enabled(true);

    bo_sdk_init(
        "BEZAVGGW4GZZZ3N".to_string(),
        "http://stage.blotout.io".to_string(),
        "com.blotout.rustsaleDemoApp".to_string(),
    )
    .await;

    let data = "{\"some property\": \"some value\", \"some other property\": \"some other value\"}"
        .to_string();
    bo_log_event("rust_event".to_string(), data).await;

    let map_data = "{\"some property\": \"some value\", \"some other property\": \"some other value\"}"
        .to_string();

    bo_map_id("abcd".to_string(), "google".to_string(), map_data).await;

    let pii_data =
        "{\"email id\": \"ankuradhikari08@gmail.com\", \"gender\": \"male\"}".to_string();
    bo_log_pii_event("PII Event".to_string(), pii_data).await;

    let phi_data = "{\"email id\": \"ankur@blotout.io\", \"gender\": \"male\"}".to_string();
    bo_log_phi_event("PHI Event".to_string(), phi_data).await;
}

fn bo_log_enabled(log_enabled: bool) {
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_log_enabled(log_enabled);
}

async fn bo_sdk_init(token: String, end_point: String, bundle_id: String) -> bool {
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

async fn bo_start_session() -> bool {
    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let response = client.send_session_start().await;
    response.is_ok()
}

async fn bo_end_session() -> bool {
    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let response = client.send_session_end().await;
    response.is_ok()
}

async fn bo_log_event(event_name: String, data: String) -> bool {
    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    let response = client.send_event(event_name.as_str(), data, 0).await;
    response.is_ok()
}

async fn bo_log_pii_event(event_name: String, data: String) -> bool {
    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    let response = client.send_pii_event(event_name.as_str(), data).await;
    response.is_ok()
}

async fn bo_log_phi_event(event_name: String, data: String) -> bool {
    let client = BOHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    //let data =json!(data);
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
    map_object.insert("map_id".to_string(), serde_json::Value::String(id));
    map_object.insert(
        "map_provider".to_string(),
        serde_json::Value::String(provider),
    );

    let payload: Value = Value::Object(map_object);

    let response = client.send_event("map_id", payload, 21001).await;
    response.is_ok()
}
