use crate::model::event::BoEventSecureDataModel;
use crate::model::manifest::ManifestVariable;
use async_trait::async_trait;
use failure::Error;
use serde_json::Value;
#[async_trait]
pub trait BoEventSecureDataApi {
    //method for sending pii events
    // event_info could be a json string e.g. let data ="{\"some property\": \"some value\", \"some other property\": \"some other value\"}".to_string();
    async fn send_pii_event(&self, event_name: &str, event_info: Value) -> Result<(), Error>;

    //method for sending pii events
    // event_info could be a json string e.g. let data ="{\"some property\": \"some value\", \"some other property\": \"some other value\"}".to_string();
    async fn send_phi_event(&self, event_name: &str, event_info: Value) -> Result<(), Error>;

    //return manifest variable based on name
    fn get_manifest_variable(&self, manifest_var_name: String) -> ManifestVariable;

    //publih secure events to server
    async fn publish_secure_events(&self, event_model: BoEventSecureDataModel)
        -> Result<(), Error>;
}
