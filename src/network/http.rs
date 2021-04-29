use crate::common::shared_manager::BOSHAREDINSTANCE;
use crate::common::system_info_manager::BOSYSTEMINFOINSTANCE;
use crate::common::utils::get_timezone_offset;
use crate::model::manifest::ManifestRoot;
use crate::model::payload::{Event, Meta, Payload};

use async_trait::async_trait;
use failure::Error;
use reqwest::header;
use std::vec::Vec;

pub struct HttpClient {
    client: reqwest::Client,
    host: String,
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient {
            client: reqwest::Client::builder().build().unwrap(),
            host: "".to_owned(),
        }
    }
}

impl HttpClient {
    pub fn new(client: reqwest::Client, host: String) -> HttpClient {
        HttpClient { client, host }
    }
}
#[async_trait]
pub trait Http {
    async fn get_manifest(&self) -> ManifestRoot;
    async fn post_events(&self, events: Vec<Event>) -> Result<(), Error>;
}

#[async_trait]
impl Http for HttpClient {
    async fn get_manifest(&self) -> ManifestRoot {
        let token_str = BOSHAREDINSTANCE.lock().unwrap().token.to_string();
        let path = "/v1/manifest/pull";

        let response = self
            .client
            .post(&format!("{}{}", self.host, path))
            .query(&[("token", token_str)])
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap()
            .json::<ManifestRoot>()
            .await
            .unwrap();

        if BOSHAREDINSTANCE.lock().unwrap().log_enabled {
            println!("---------------- Manifest ------------------");
            println!("{}", serde_json::to_string_pretty(&response).unwrap());
        }

        response
    }

    async fn post_events(&self, events: Vec<Event>) -> Result<(), Error> {
        let plf_code = BOSYSTEMINFOINSTANCE.lock().unwrap().platform_code;
        let event_model = Payload {
            meta: Meta {
                osn: BOSYSTEMINFOINSTANCE.lock().unwrap().os_type.to_string(),
                plf: plf_code,
                sdkv: env!("CARGO_PKG_VERSION").to_string(),
                tz_offset: get_timezone_offset(),
                user_id_created: BOSHAREDINSTANCE.lock().unwrap().user_id_created,
            },
            events,
        };

        if BOSHAREDINSTANCE.lock().unwrap().log_enabled {
            println!("----------------- Event ------------");
            println!("{}", serde_json::to_string_pretty(&event_model).unwrap());
        }

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
            println!("{:?}", response);
        }

        Ok(())
    }
}
