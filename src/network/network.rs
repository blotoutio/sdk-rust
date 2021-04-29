use crate::model::event::BoEvent;
use crate::model::event::BoEventModel;
use crate::model::event::BoEventSecureDataModel;
use crate::model::event::BoMeta;
use crate::model::event::BoPropertiesInfo;
use crate::model::event::BoSecureData;
use crate::model::manifest::ManifestRoot;
use crate::model::manifest::ManifestVariable;
use crate::network::event_api::EventApi;
use crate::network::event_personal_api::BoEventSecureDataApi;
use crate::network::manifest_api::BoManifestApi;
use crate::utility::common_utility::BOSHAREDCOMMONUTILITYINSTANCE;
use crate::utility::file_manager::BOSHAREDFILEINSTANCE;
use crate::utility::shared_manager::BOSHAREDINSTANCE;
use crate::utility::system_info_manager::BOSYSTEMINFOINSTANCE;
use async_trait::async_trait;
use chrono::Utc;
use failure::Error;
use reqwest::header;
use serde_json::{json, Value};
use std::vec::Vec;

const BO_CRYPTO_IV: &str = "Q0BG17E2819IWZYQ";
const BO_EVENT_SDK_START: u64 = 11130;
const BO_SDK_START: &str = "sdk_start";

pub struct BoHttpClient {
    client: reqwest::Client,
    host: String,
}

impl Default for BoHttpClient {
    fn default() -> Self {
        BoHttpClient {
            client: reqwest::Client::builder().build().unwrap(),
            host: "".to_owned(),
        }
    }
}

impl BoHttpClient {
    /// Construct a new `HttpClient` from a `reqwest::Client`
    pub fn new(client: reqwest::Client, host: String) -> BoHttpClient {
        BoHttpClient { client, host }
    }
}

#[async_trait]
impl BoManifestApi for BoHttpClient {
    async fn get_manifest(&self) -> Result<(), Error> {
        let token_str = BOSHAREDINSTANCE.lock().unwrap().token.to_string();
        let path = "/v1/manifest/pull";

        let response = self
            .client
            .post(&format!("{}{}", self.host, path))
            .query(&[("token", token_str)])
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await?
            .json::<ManifestRoot>()
            .await?;

        BOSHAREDINSTANCE
            .lock()
            .unwrap()
            .set_manifest(response.to_owned());
        BOSHAREDINSTANCE
            .lock()
            .unwrap()
            .set_sdk_enabled(!response.variables.is_empty());

        Ok(())
    }
}

#[async_trait]
impl EventApi for BoHttpClient {
    async fn send_event(
        &self,
        event_name: &str,
        event_info: Value,
        event_code: u64,
    ) -> Result<(), Error> {
        if !BOSHAREDINSTANCE.lock().unwrap().sdk_enabled {
            return Ok(());
        }

        let mut event_sub_code = event_code;

        if event_code == 0 {
            event_sub_code = BOSHAREDCOMMONUTILITYINSTANCE
                .lock()
                .unwrap()
                .code_for_custom_codified_event(event_name.to_string());
        }

        let mut events_arr: Vec<BoEvent> = Vec::new();
        let event_properties = BoPropertiesInfo {
            codified_info: event_info,
            session_id: BOSHAREDINSTANCE.lock().unwrap().session_id.to_string(),
        };

        let event_model = BoEvent {
            evn: event_name.to_string(),
            properties: event_properties,
            evcs: event_sub_code,
            evt: Utc::now().timestamp_millis(),
            userid: BOSHAREDFILEINSTANCE.lock().unwrap().get_user_id(),
            ..Default::default()
        };

        events_arr.push(event_model);

        let event_model = self.get_payload(events_arr);

        if BOSHAREDINSTANCE.lock().unwrap().log_enabled {
            println!(
                "-----------------Event model to be posted:------------{:?}",
                event_model
            );
        }

        let response = self.publish_events(event_model).await;

        response
    }

    async fn send_sdk_start(&self) -> Result<(), Error> {
        if !BOSHAREDINSTANCE.lock().unwrap().sdk_enabled {
            return Ok(());
        }

        let mut events_arr: Vec<BoEvent> = Vec::new();
        let event_properties = BoPropertiesInfo {
            session_id: BOSHAREDINSTANCE.lock().unwrap().session_id.to_string(),
            ..Default::default()
        };

        let event_model = BoEvent {
            evn: BO_SDK_START.to_string(),
            evcs: BO_EVENT_SDK_START,
            evt: Utc::now().timestamp_millis(),
            userid: BOSHAREDFILEINSTANCE.lock().unwrap().get_user_id(),
            properties: event_properties,
            ..Default::default()
        };

        events_arr.push(event_model);

        let event_model = self.get_payload(events_arr);

        if BOSHAREDINSTANCE.lock().unwrap().log_enabled {
            println!(
                "-----------------Event Session Start Model to be posted:------------{:?}",
                event_model
            );
        }

        let response = self.publish_events(event_model).await;

        response
    }

    fn get_payload(&self, events_arr: Vec<BoEvent>) -> BoEventModel {
        let plf_code: i64 = BOSYSTEMINFOINSTANCE.lock().unwrap().platform_code;
        let sdk_version = env!("CARGO_PKG_VERSION").to_string();

        let event_model = BoEventModel {
            meta: BoMeta {
                osn: BOSYSTEMINFOINSTANCE.lock().unwrap().os_type.to_string(),
                plf: plf_code,
                sdkv: sdk_version,
                tz_offset: BOSHAREDCOMMONUTILITYINSTANCE
                    .lock()
                    .unwrap()
                    .get_timezone_offset(),
            },
            events: events_arr,
        };

        event_model
    }

    async fn publish_events(&self, event_model: BoEventModel) -> Result<(), Error> {
        let token_str = BOSHAREDINSTANCE.lock().unwrap().token.to_string();
        let path = "/v1/events/publish";

        let response = self
            .client
            .post(&format!("{}{}", self.host, path))
            .query(&[("token", token_str)])
            .header(header::CONTENT_TYPE, "application/json")
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
impl BoEventSecureDataApi for BoHttpClient {
    fn get_manifest_variable(&self, manifest_var_name: String) -> ManifestVariable {
        let manifest: ManifestRoot = BOSHAREDINSTANCE.lock().unwrap().manifest.to_owned();

        for manifest_var in manifest.variables {
            if manifest_var.variable_name.eq(manifest_var_name.as_str()) {
                return manifest_var;
            }
        }

        ManifestVariable::default()
    }

    async fn send_pii_event(&self, event_name: &str, event_info: Value) -> Result<(), Error> {
        if !BOSHAREDINSTANCE.lock().unwrap().sdk_enabled {
            return Ok(());
        }

        let mut events_arr: Vec<BoEvent> = Vec::new();
        let event_properties = BoPropertiesInfo {
            codified_info: event_info,
            session_id: BOSHAREDINSTANCE.lock().unwrap().session_id.to_string(),
        };

        let event_model = BoEvent {
            evn: event_name.to_string(),
            properties: event_properties,
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
        let encrypted_string = base64::encode(encrypted_data.unwrap());

        //RSA key encryption
        let pii_manifest_variable: ManifestVariable =
            self.get_manifest_variable("PII_Public_Key".to_string());
        let encrypted_rsa_key = BOSHAREDCOMMONUTILITYINSTANCE
            .lock()
            .unwrap()
            .encrypt_key_using_rsa(uuid, pii_manifest_variable.value);

        //preparing final model
        let plf_code: i64 = BOSYSTEMINFOINSTANCE.lock().unwrap().platform_code;
        let sdk_version = env!("CARGO_PKG_VERSION").to_string();

        let event_data_model = BoEventSecureDataModel {
            meta: BoMeta {
                osn: BOSYSTEMINFOINSTANCE.lock().unwrap().os_type.to_string(),
                plf: plf_code,
                sdkv: sdk_version,
                tz_offset: BOSHAREDCOMMONUTILITYINSTANCE
                    .lock()
                    .unwrap()
                    .get_timezone_offset(),
            },
            pii: BoSecureData {
                key: encrypted_rsa_key,
                data: encrypted_string,
                ..Default::default()
            },
            phi: BoSecureData {
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
        event_model: BoEventSecureDataModel,
    ) -> Result<(), Error> {
        let token_str = BOSHAREDINSTANCE.lock().unwrap().token.to_string();
        let path = "/v1/events/publish";

        let response = self
            .client
            .post(&format!("{}{}", self.host, path))
            .query(&[("token", token_str)])
            .header(header::CONTENT_TYPE, "application/json")
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
        if !BOSHAREDINSTANCE.lock().unwrap().sdk_enabled {
            return Ok(());
        }

        let mut events_arr: Vec<BoEvent> = Vec::new();
        let event_properties = BoPropertiesInfo {
            codified_info: event_info,
            session_id: BOSHAREDINSTANCE.lock().unwrap().session_id.to_string(),
        };

        let event_model = BoEvent {
            evn: event_name.to_string(),
            properties: event_properties,
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
        let phi_manifest_variable: ManifestVariable =
            self.get_manifest_variable("PHI_Public_Key".to_string());
        let encrypted_rsa_key = BOSHAREDCOMMONUTILITYINSTANCE
            .lock()
            .unwrap()
            .encrypt_key_using_rsa(uuid, phi_manifest_variable.value);

        //preparing final model
        let plf_code: i64 = BOSYSTEMINFOINSTANCE.lock().unwrap().platform_code;
        let sdk_version = env!("CARGO_PKG_VERSION").to_string();

        let event_data_model = BoEventSecureDataModel {
            meta: BoMeta {
                osn: BOSYSTEMINFOINSTANCE.lock().unwrap().os_type.to_string(),
                plf: plf_code,
                sdkv: sdk_version,
                tz_offset: BOSHAREDCOMMONUTILITYINSTANCE
                    .lock()
                    .unwrap()
                    .get_timezone_offset(),
            },
            phi: BoSecureData {
                key: encrypted_rsa_key,
                data: encrypted_string,
                ..Default::default()
            },
            pii: BoSecureData {
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
