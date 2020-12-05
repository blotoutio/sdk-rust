use crate::model::boeventmodel::BOEventModel;
use async_trait::async_trait;
use failure::Error;
use serde_json::Value;

#[async_trait]
pub trait BOEventAPI {
    /// Send a single message to Blotout.
    async fn send_event(&self, event_name: &str, event_info: Value) -> Result<(), Error>;

    //send session start event on sdk initilization
    async fn send_session_start(&self) -> Result<(), Error>;

    //send session sent when app exits, user have to call this explicitly
    async fn send_session_end(&self) -> Result<(), Error>;

    //method to push events using given model
    async fn publish_events(&self, event_model: BOEventModel) -> Result<(), Error>;
}