use blotout::common::api::{
    capture, capture_personal, enable_log, enable_sdk, get_user_id, init, map_id,
};
use blotout::model::map_id::MapIDData;

#[tokio::main]
async fn main() {
    bo_enable_log(true);

    bo_init(
        "7T3VGKRTMZND4Q9".to_string(),
        "http://stage.blotout.io/sdk".to_string(),
    )
    .await;

    let mut event_name = "add_to_cart".to_string();
    let mut screen_name = "home".to_string();
    let mut data = r#"{
        "item": "phone"
    }"#
    .to_string();
    bo_capture(event_name, data, screen_name).await;

    event_name = "user_registration".to_string();
    screen_name = "registration".to_string();
    data = r#"{
        "email": "user@example.com",
        "gender": "female"
    }"#
    .to_string();
    bo_capture_personal(event_name, data, false, screen_name).await;

    event_name = "blood_group".to_string();
    screen_name = "signup".to_string();
    data = r#"{
        "bloodGroup": "A+ve"
    }"#
    .to_string();
    bo_capture_personal(event_name, data, true, screen_name).await;

    let map_data = MapIDData {
        external_id: "2f28023hj0-2323-23232".to_string(),
        provider: "service".to_string(),
    };

    data = r#"{
        "lang": "en"
    }"#
    .to_string();
    bo_map_id(map_data, data).await;

    println!("User ID: {}", bo_get_user_id());
}

pub async fn bo_init(token: String, endpoint_url: String) -> bool {
    init(token, endpoint_url).await
}

pub async fn bo_capture(event_name: String, data: String, screen_name: String) -> bool {
    capture(event_name, data, screen_name).await
}

pub async fn bo_capture_personal(
    event_name: String,
    data: String,
    is_phi: bool,
    screen_name: String,
) -> bool {
    capture_personal(event_name, data, is_phi, screen_name).await
}

pub async fn bo_map_id(map_id_data: MapIDData, data: String) -> bool {
    map_id(map_id_data, data).await
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
