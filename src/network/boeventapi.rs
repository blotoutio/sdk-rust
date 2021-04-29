use crate::model::boeventmodel::BOEventModel;
use async_trait::async_trait;
use failure::Error;
use serde_json::Value;

#[async_trait]
pub trait BOEventAPI {
    /// Send a single message to Blotout.
    async fn send_event(
        &self,
        event_name: &str,
        event_info: Value,
        event_code: u64,
    ) -> Result<(), Error>;

    //send start event on sdk initialization
    async fn send_sdk_start(&self) -> Result<(), Error>;

    //get final payload
    fn get_payload(&self, events: Vec<BOEvent>) -> BOEventModel;

    //method to push events using given model
    async fn publish_events(&self, event_model: BOEventModel) -> Result<(), Error>;
}
