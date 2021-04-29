use blotout::network::boeventapi::BoEventApi;
use blotout::network::boeventsecuredataapi::BoEventSecureDataApi;
use blotout::network::bohttp::BoHttpClient;
use blotout::network::bomanifestapi::BoManifestApi;
use blotout::utility::bosharedmanager::BOSHAREDINSTANCE;
use blotout::utility::bosysteminfomanager::BOSYSTEMINFOINSTANCE;
use serde_json::Value;

const BO_EVENT_MAP_ID: u64 = 21001;
const BO_MAP_ID: &str = "map_id";
const BO_MAP_PROVIDER: &str = "map_provider";

#[tokio::main]
async fn main() {
    bo_log_enabled(true);

    bo_init(
        "7T3VGKRTMZND4Q9".to_string(),
        "http://stage.blotout.io/sdk".to_string(),
    )
    .await;

    let data = "{\"some property\": \"some value\", \"some other property\": \"some other value\"}"
        .to_string();
    bo_log_event("rust_event".to_string(), data).await;

    let map_data =
        "{\"some property\": \"some value\", \"some other property\": \"some other value\"}"
            .to_string();

    bo_map_id("abcd".to_string(), "google".to_string(), map_data).await;

    let pii_data =
        "{\"email id\": \"ankuradhikari08@gmail.com\", \"gender\": \"male\"}".to_string();
    bo_log_pii_event("PII Event".to_string(), pii_data).await;

    let phi_data = "{\"email id\": \"ankur@blotout.io\", \"gender\": \"male\"}".to_string();
    bo_log_phi_event("PHI Event".to_string(), phi_data).await;
}

async fn bo_init(token: String, endpoint_url: String) -> bool {
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
        let session_response = client.send_session_start().await;
        println!("{:?}", session_response.is_err());
        true
    } else {
        false
    }
}

async fn bo_log_event(event_name: String, data: String) -> bool {
    let client = BoHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    let response = client.send_event(event_name.as_str(), data, 0).await;
    response.is_ok()
}

async fn bo_log_pii_event(event_name: String, data: String) -> bool {
    let client = BoHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    let response = client.send_pii_event(event_name.as_str(), data).await;
    response.is_ok()
}

async fn bo_log_phi_event(event_name: String, data: String) -> bool {
    let client = BoHttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let data: Value = serde_json::from_str(data.as_str()).unwrap();
    //let data =json!(data);
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

fn bo_log_enabled(log_enabled: bool) {
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_log_enabled(log_enabled);
}
