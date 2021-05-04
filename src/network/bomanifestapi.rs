use async_trait::async_trait;
use failure::Error;

#[async_trait]
pub trait BOManifestAPI {
    //fetch manifest configuration from the server
    async fn get_manifest(&self) -> Result<(), Error>;
}
