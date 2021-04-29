use crate::model::bomanifestmodel::BOManifestRoot;
use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

#[derive(Default)]
pub struct BOSharedManager {
    pub manifest: BOManifestRoot,
    pub token: String,
    pub base_url: String,
    pub log_enabled: bool,
    pub user_id: String,
    pub sdk_enabled: bool,
    pub session_id: String,
}

impl BOSharedManager {
    pub fn set_manifest(&mut self, newmanifest: BOManifestRoot) {
        self.manifest = newmanifest;
    }

    pub fn set_token(&mut self, newtoken: String) {
        self.token = newtoken;
    }

    pub fn set_base_url(&mut self, newbaseurl: String) {
        self.base_url = newbaseurl;
    }

    pub fn set_log_enabled(&mut self, logenabled: bool) {
        self.log_enabled = logenabled;
    }

    pub fn set_user_id(&mut self, userid: String) {
        self.user_id = userid;
    }

    pub fn set_sdk_enabled(&mut self, sdk_enabled: bool) {
        self.sdk_enabled = sdk_enabled;
    }

    pub fn set_session_id(&mut self, sessionid: String) {
        self.session_id = sessionid;
    }
}

lazy_static! {
    pub static ref BOSHAREDINSTANCE: Mutex<BOSharedManager> =
        Mutex::new(BOSharedManager::default());
}
