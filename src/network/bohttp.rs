use crate::model::boeventmodel::BOEvent;
use crate::model::boeventmodel::BOEventModel;
use crate::model::boeventmodel::BOEventSecureDataModel;
use crate::model::boeventmodel::BOGeo;
use crate::model::boeventmodel::BOMeta;
use crate::model::boeventmodel::BOSecureData;
use crate::model::bomanifestmodel::BOManifestRoot;
use crate::model::bomanifestmodel::BOManifestVariable;
use crate::network::boeventapi::BOEventAPI;
use crate::network::boeventsecuredataapi::BOEventSecureDataAPI;
use crate::network::bomanifestapi::BOManifestAPI;
use crate::utility::bocommonutility::BOSHAREDCOMMONUTILITYINSTANCE;
use crate::utility::bofilemanager::BOSHAREDFILEINSTANCE;
use crate::utility::bosharedmanager::BOSHAREDINSTANCE;
use crate::utility::bosysteminfomanager::BOSYSTEMINFOINSTANCE;
use async_trait::async_trait;
use chrono::Utc;
use failure::Error;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::vec::Vec;

const BO_CRYPTO_IV: &str = "Q0BG17E2819IWZYQ";

pub struct BOHttpClient {
    client: reqwest::Client,
    host: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BOManifestRequestModel {
    #[serde(rename = "lastUpdatedTime")]
    pub last_updated_time: i64,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
}

impl Default for BOHttpClient {
    fn default() -> Self {
        BOHttpClient {
            client: reqwest::Client::builder().build().unwrap(),
            host: "http://dev.blotout.io".to_owned(),
        }
    }
}

impl BOHttpClient {
    /// Construct a new `HttpClient` from a `reqwest::Client`
    pub fn new(client: reqwest::Client, host: String) -> BOHttpClient {
        BOHttpClient { client, host }
    }
}

#[async_trait]
impl BOManifestAPI for BOHttpClient {
    async fn get_manifest(&self) -> Result<(), Error> {
        let path = "/sdk/v1/manifest/pull";
        let bundle_id_str = BOSHAREDINSTANCE.lock().unwrap().bundle_id.to_string();
        let manifest_request = BOManifestRequestModel {
            last_updated_time: 0,
            bundle_id: bundle_id_str,
        };

        let token_str = BOSHAREDINSTANCE.lock().unwrap().token.to_string();
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_lowercase(b"token").unwrap(),
            HeaderValue::from_str(token_str.as_str()).unwrap(),
        );
        headers.insert(
            HeaderName::from_lowercase(b"content-type").unwrap(),
            HeaderValue::from_str("application/json").unwrap(),
        );
        headers.insert(
            HeaderName::from_lowercase(b"version").unwrap(),
            HeaderValue::from_str("v1").unwrap(),
        );

        let response = self
            .client
            .post(&format!("{}{}", self.host, path))
            .headers(headers)
            .json(&manifest_request)
            .send()
            .await?
            .json::<BOManifestRoot>()
            .await?;

        BOSHAREDINSTANCE.lock().unwrap().set_manifest(response);

        Ok(())
    }
}

#[async_trait]
impl BOEventAPI for BOHttpClient {
    async fn send_event(&self, event_name: &str, event_info: Value) -> Result<(), Error> {
        let mut events_arr: Vec<BOEvent> = Vec::new();

        let event_model = BOEvent {
            evn: event_name.to_string(),
            properties: event_info,
            evc: 20001,
            evcs: BOSHAREDCOMMONUTILITYINSTANCE
                .lock()
                .unwrap()
                .code_for_custom_codified_event(event_name.to_string()),
            evt: Utc::now().timestamp_millis(),
            userid: BOSHAREDFILEINSTANCE.lock().unwrap().get_user_id(),
            ..Default::default()
        };

        events_arr.push(event_model);

        let plf_code: i64 = BOSYSTEMINFOINSTANCE.lock().unwrap().platform_code;
        let event_model = BOEventModel {
            geo: BOGeo {
                ..Default::default()
            },
            meta: BOMeta {
                osn: BOSYSTEMINFOINSTANCE.lock().unwrap().os_type.to_string(),
                plf: plf_code,
                ..Default::default()
            },
            events: events_arr,
        };

        if BOSHAREDINSTANCE.lock().unwrap().log_enabled {
            println!(
                "-----------------Event model to be posted:------------{:?}",
                event_model
            );
        }

        let response = self.publish_events(event_model).await;

        response
    }

    async fn send_session_start(&self) -> Result<(), Error> {
        let mut events_arr: Vec<BOEvent> = Vec::new();

        let event_model = BOEvent {
            evn: "Session Start".to_string(),
            evc: 10001,
            evcs: 11011,
            evt: Utc::now().timestamp_millis(),
            userid: BOSHAREDFILEINSTANCE.lock().unwrap().get_user_id(),
            ..Default::default()
        };

        events_arr.push(event_model);

        let plf_code: i64 = BOSYSTEMINFOINSTANCE.lock().unwrap().platform_code;
        let event_model = BOEventModel {
            geo: BOGeo {
                ..Default::default()
            },
            meta: BOMeta {
                osn: BOSYSTEMINFOINSTANCE.lock().unwrap().os_type.to_string(),
                plf: plf_code,
                ..Default::default()
            },
            events: events_arr,
        };

        if BOSHAREDINSTANCE.lock().unwrap().log_enabled {
            println!(
                "-----------------Event Session Start Model to be posted:------------{:?}",
                event_model
            );
        }

        let response = self.publish_events(event_model).await;

        response
    }

    async fn send_session_end(&self) -> Result<(), Error> {
        let mut events_arr: Vec<BOEvent> = Vec::new();

        let event_model = BOEvent {
            evn: "Session End".to_string(),
            evc: 10001,
            evcs: 11012,
            evt: Utc::now().timestamp_millis(),
            userid: BOSHAREDFILEINSTANCE.lock().unwrap().get_user_id(),
            ..Default::default()
        };

        events_arr.push(event_model);

        let plf_code: i64 = BOSYSTEMINFOINSTANCE.lock().unwrap().platform_code;
        let event_model = BOEventModel {
            geo: BOGeo {
                ..Default::default()
            },
            meta: BOMeta {
                osn: BOSYSTEMINFOINSTANCE.lock().unwrap().os_type.to_string(),
                plf: plf_code,
                ..Default::default()
            },
            events: events_arr,
        };

        if BOSHAREDINSTANCE.lock().unwrap().log_enabled {
            println!(
                "-----------------Event Session End Model to be posted:------------{:?}",
                event_model
            );
        }

        let response = self.publish_events(event_model).await;

        response
    }

    async fn publish_events(&self, event_model: BOEventModel) -> Result<(), Error> {
        let path = "/sdk/v1/events/publish";

        let mut headers = HeaderMap::new();
        let token_str = BOSHAREDINSTANCE.lock().unwrap().token.to_string();
        headers.insert(
            HeaderName::from_lowercase(b"token").unwrap(),
            HeaderValue::from_str(token_str.as_str()).unwrap(),
        );
        headers.insert(
            HeaderName::from_lowercase(b"content-type").unwrap(),
            HeaderValue::from_str("application/json").unwrap(),
        );
        headers.insert(
            HeaderName::from_lowercase(b"version").unwrap(),
            HeaderValue::from_str("v1").unwrap(),
        );

        let response = self
            .client
            .post(&format!("{}{}", self.host, path))
            .headers(headers)
            .json(&event_model)
            .send()
            .await;

        if BOSHAREDINSTANCE.lock().unwrap().log_enabled {
            println!("----------------Event Developer codified Events Posted response:------------------{:?}",response);
        }

        Ok(())
    }
}

#[async_trait]
impl BOEventSecureDataAPI for BOHttpClient {
    fn get_manifest_variable(&self, manifest_var_name: String) -> BOManifestVariable {
        let manifest: BOManifestRoot = BOSHAREDINSTANCE.lock().unwrap().manifest.to_owned();

        for manifest_var in manifest.variables {
            if manifest_var.variable_name.eq(manifest_var_name.as_str()) {
                return manifest_var;
            }
        }

        BOManifestVariable::default()
    }

    async fn send_pii_event(&self, event_name: &str, event_info: Value) -> Result<(), Error> {
        let mut events_arr: Vec<BOEvent> = Vec::new();

        let event_model = BOEvent {
            evn: event_name.to_string(),
            properties: event_info,
            evc: 20001,
            evcs: BOSHAREDCOMMONUTILITYINSTANCE
                .lock()
                .unwrap()
                .code_for_custom_codified_event(event_name.to_string()),
            evt: Utc::now().timestamp_millis(),
            userid: BOSHAREDFILEINSTANCE.lock().unwrap().get_user_id(),
            ..Default::default()
        };

        events_arr.push(event_model);

        let data = json!(events_arr).to_string();
        //public static final String Event_PHI_Public_Key = "PHI_Public_Key";
        //public static final String Event_PII_Public_Key = "PII_Public_Key";

        //AES data encryption
        let uuid: String = BOSHAREDCOMMONUTILITYINSTANCE
            .lock()
            .unwrap()
            .generate_user_id();
        let iv_string = BO_CRYPTO_IV;
        let encrypted_data = BOSHAREDCOMMONUTILITYINSTANCE
            .lock()
            .unwrap()
            .encrypt_data_using_aes(data.as_bytes(), uuid.as_bytes(), iv_string.as_bytes());
        let encrypted_string = base64::encode(encrypted_data.unwrap());

        //RSA key encryption
        let pii_manifest_variable: BOManifestVariable =
            self.get_manifest_variable("PII_Public_Key".to_string());
        let encrypted_rsa_key = BOSHAREDCOMMONUTILITYINSTANCE
            .lock()
            .unwrap()
            .encrypt_key_using_rsa(uuid, pii_manifest_variable.value);

        //preparing final model
        let plf_code: i64 = BOSYSTEMINFOINSTANCE.lock().unwrap().platform_code;
        let event_data_model = BOEventSecureDataModel {
            geo: BOGeo {
                ..Default::default()
            },
            meta: BOMeta {
                osn: BOSYSTEMINFOINSTANCE.lock().unwrap().os_type.to_string(),
                plf: plf_code,
                ..Default::default()
            },
            pii: BOSecureData {
                key: encrypted_rsa_key,
                data: encrypted_string,
                ..Default::default()
            },
            phi: BOSecureData {
                ..Default::default()
            },
        };

        if BOSHAREDINSTANCE.lock().unwrap().log_enabled {
            println!(
                "-----------------Event PII Data Model to be posted:------------{:?}",
                event_data_model
            );
        }

        let response = self.publish_secure_events(event_data_model).await;

        response
    }

    async fn publish_secure_events(
        &self,
        event_model: BOEventSecureDataModel,
    ) -> Result<(), Error> {
        let path = "/sdk/v1/events/publish";

        let mut headers = HeaderMap::new();
        let token_str = BOSHAREDINSTANCE.lock().unwrap().token.to_string();
        headers.insert(
            HeaderName::from_lowercase(b"token").unwrap(),
            HeaderValue::from_str(token_str.as_str()).unwrap(),
        );
        headers.insert(
            HeaderName::from_lowercase(b"content-type").unwrap(),
            HeaderValue::from_str("application/json").unwrap(),
        );
        headers.insert(
            HeaderName::from_lowercase(b"version").unwrap(),
            HeaderValue::from_str("v1").unwrap(),
        );

        let response = self
            .client
            .post(&format!("{}{}", self.host, path))
            .headers(headers)
            .json(&event_model)
            .send()
            .await;

        if BOSHAREDINSTANCE.lock().unwrap().log_enabled {
            println!(
                "----------------Event Secure Events Posted response:------------------{:?}",
                response
            );
        }

        Ok(())
    }

    async fn send_phi_event(&self, event_name: &str, event_info: Value) -> Result<(), Error> {
        let mut events_arr: Vec<BOEvent> = Vec::new();

        let event_model = BOEvent {
            evn: event_name.to_string(),
            properties: event_info,
            evc: 20001,
            evcs: BOSHAREDCOMMONUTILITYINSTANCE
                .lock()
                .unwrap()
                .code_for_custom_codified_event(event_name.to_string()),
            evt: Utc::now().timestamp_millis(),
            userid: BOSHAREDFILEINSTANCE.lock().unwrap().get_user_id(),
            ..Default::default()
        };

        events_arr.push(event_model);

        let data = json!(events_arr).to_string();

        //AES data encryption
        let uuid: String = BOSHAREDCOMMONUTILITYINSTANCE
            .lock()
            .unwrap()
            .generate_user_id();
        let iv_string = BO_CRYPTO_IV;
        let encrypted_data = BOSHAREDCOMMONUTILITYINSTANCE
            .lock()
            .unwrap()
            .encrypt_data_using_aes(data.as_bytes(), uuid.as_bytes(), iv_string.as_bytes());
        let encrypted_string = base64::encode(encrypted_data.unwrap()); // String::from_utf8(encrypted_data.unwrap());

        //RSA key encryption
        let phi_manifest_variable: BOManifestVariable =
            self.get_manifest_variable("PHI_Public_Key".to_string());
        let encrypted_rsa_key = BOSHAREDCOMMONUTILITYINSTANCE
            .lock()
            .unwrap()
            .encrypt_key_using_rsa(uuid, phi_manifest_variable.value);

        //preparing final model
        let plf_code: i64 = BOSYSTEMINFOINSTANCE.lock().unwrap().platform_code;
        let event_data_model = BOEventSecureDataModel {
            geo: BOGeo {
                ..Default::default()
            },
            meta: BOMeta {
                osn: BOSYSTEMINFOINSTANCE.lock().unwrap().os_type.to_string(),
                plf: plf_code,
                ..Default::default()
            },
            phi: BOSecureData {
                key: encrypted_rsa_key,
                data: encrypted_string,
                ..Default::default()
            },
            pii: BOSecureData {
                ..Default::default()
            },
        };

        if BOSHAREDINSTANCE.lock().unwrap().log_enabled {
            println!(
                "-----------------Event PHI Data Model to be posted:------------{:?}",
                event_data_model
            );
        }

        let response = self.publish_secure_events(event_data_model).await;

        response
    }
}
