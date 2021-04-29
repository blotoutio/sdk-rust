use crate::common::api::{
    capture, capture_personal, enable_log, enable_sdk, get_user_id, init, map_id,
};

#[tokio::main]
async fn main() {
    bo_enable_log(true);

    bo_init(
        "7T3VGKRTMZND4Q9".to_string(),
        "http://stage.blotout.io/sdk".to_string(),
    )
    .await;

    let data = "{\"some property\": \"some value\", \"some other property\": \"some other value\"}"
        .to_string();
    bo_capture("rust_event".to_string(), data).await;

    let map_data =
        "{\"some property\": \"some value\", \"some other property\": \"some other value\"}"
            .to_string();

    bo_map_id("abcd".to_string(), "google".to_string(), map_data).await;

    let pii_data =
        "{\"email id\": \"ankuradhikari08@gmail.com\", \"gender\": \"male\"}".to_string();
    bo_capture_personal("PII Event".to_string(), pii_data, false).await;

    let phi_data = "{\"email id\": \"ankur@blotout.io\", \"gender\": \"male\"}".to_string();
    bo_capture_personal("PHI Event".to_string(), phi_data, true).await;

    bo_end_session().await;
}

pub async fn bo_init(token: String, endpoint_url: String) -> bool {
    init(token, endpoint_url).await
}

pub async fn bo_capture(event_name: String, data: String) -> bool {
    capture(event_name, data).await
}

pub async fn bo_capture_personal(event_name: String, data: String, is_phi: bool) -> bool {
    capture_personal(event_name, data, is_phi).await
}

pub async fn bo_map_id(external_id: String, provider: String, data: String) -> bool {
    map_id(external_id, provider, data)
}

pub fn bo_enable_log(enable: bool) {
    enable_log(enable)
}

pub fn bo_get_user_id() -> String {
    get_user_id()
}

pub fn bo_enable(enable: bool) {
    enable_sdk(enable)
}
