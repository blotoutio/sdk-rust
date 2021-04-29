use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Storage {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userIdCreated")]
    pub user_id_created: i64,
}
