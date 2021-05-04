use crate::model::boeventmodel::BOEventSecureDataModel;
use crate::model::bomanifestmodel::BOManifestVariable;
use async_trait::async_trait;
use failure::Error;
use serde_json::Value;
#[async_trait]
pub trait BOEventSecureDataAPI {
    //method for sending pii events
    // event_info could be a json string e.g. let data ="{\"some property\": \"some value\", \"some other property\": \"some other value\"}".to_string();
    async fn send_pii_event(&self, event_name: &str, event_info: Value) -> Result<(), Error>;

    //method for sending pii events
    // event_info could be a json string e.g. let data ="{\"some property\": \"some value\", \"some other property\": \"some other value\"}".to_string();
    async fn send_phi_event(&self, event_name: &str, event_info: Value) -> Result<(), Error>;

    //return manifest variable based on name
    fn get_manifest_variable(&self, manifest_var_name: String) -> BOManifestVariable;

    //publih secure events to server
    async fn publish_secure_events(&self, event_model: BOEventSecureDataModel)
        -> Result<(), Error>;
}
