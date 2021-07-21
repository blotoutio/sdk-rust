pub mod common;
pub mod model;
pub mod network;

use crate::common::api::{
    capture, capture_personal, enable_log, enable_sdk, get_user_id, init, map_id,
};
use crate::model::map_id::MapIDData;

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
