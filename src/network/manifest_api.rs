use crate::common::shared_manager::BOSHAREDINSTANCE;
use crate::model::manifest::{ManifestRoot, ManifestVariable};
use crate::network::http::{Http, HttpClient};

use failure::Error;

pub async fn get_manifest() -> Result<(), Error> {
    let client = HttpClient::new(
        reqwest::Client::new(),
        BOSHAREDINSTANCE.lock().unwrap().base_url.to_string(),
    );

    let response = client.get_manifest().await;

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

pub fn get_manifest_variable(manifest_var_name: String) -> ManifestVariable {
    let manifest: ManifestRoot = BOSHAREDINSTANCE.lock().unwrap().manifest.to_owned();

    for manifest_var in manifest.variables {
        if manifest_var.variable_name.eq(manifest_var_name.as_str()) {
            return manifest_var;
        }
    }

    ManifestVariable::default()
}
