use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BOManifestRoot {
    #[serde(rename = "variables")]
    pub variables: Vec<BOManifestVariable>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BOManifestVariable {
    #[serde(rename = "variableId")]
    pub variable_id: i64,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "variableDataType")]
    pub variable_data_type: i64,
    #[serde(rename = "variableName")]
    pub variable_name: String,
}
