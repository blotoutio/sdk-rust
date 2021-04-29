use crate::model::manifest::ManifestRoot;
use chrono::Utc;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Default)]
pub struct SharedManager {
    pub manifest: ManifestRoot,
    pub token: String,
    pub base_url: String,
    pub log_enabled: bool,
    pub user_id: String,
    pub sdk_enabled: bool,
    pub session_id: String,
    pub user_id_created: i64,
}

impl SharedManager {
    pub fn set_manifest(&mut self, value: ManifestRoot) {
        self.manifest = value;
    }

    pub fn set_token(&mut self, value: String) {
        self.token = value;
    }

    pub fn set_base_url(&mut self, value: String) {
        self.base_url = value;
    }

    pub fn set_log_enabled(&mut self, value: bool) {
        self.log_enabled = value;
    }

    pub fn set_user_id(&mut self, value: String) {
        self.user_id = value;
    }

    pub fn set_sdk_enabled(&mut self, value: bool) {
        self.sdk_enabled = value;
    }

    pub fn set_session_id(&mut self, value: String) {
        self.session_id = value;
    }

    pub fn set_user_id_created(&mut self, value: i64) {
        self.user_id_created = value;
    }
}

lazy_static! {
    pub static ref BOSHAREDINSTANCE: Mutex<SharedManager> = Mutex::new(SharedManager {
        session_id: Utc::now().timestamp_millis().to_string(),
        ..Default::default()
    });
}
