# # Defined Events


## bo_map_id
The `bo_map_id` method allows you to map external services to Blotout ID.

#### Input

||||
|---|---|---|
| `mapIDData` | `Object` | Required | See data table. |
| `event_data` | `String` | You can provide some additional data to this event. There is no limitation as this is just a key-value pair send to the server. |


#### Data

|              |          |          |                                                            |
| ------------ | -------- | -------- | ---------------------------------------------------------- |
| `external_id` | `String` | Required | External ID that you want to link to Blotout ID.           |
| `provider`   | `String` | Required | Provider that generated external ID, for example `sass` |

#### Example
```rust
use blotout::bo_map_id;
use blotout::model::map_id::MapIDData;

let map_data = MapIDData {
    external_id: "2f28023hj0-2323-23232".to_string(),
    provider: "sass".to_string(),
};

let data = r#"{
    "lang": "en"
}"#;

bo_map_id(map_data, data.to_string()).await;
```
