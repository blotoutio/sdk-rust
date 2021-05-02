# API

## bo_init
The `bo_init` method is used for initializing SDK. This sets all required configurations and also sends system event `sdk_start` which allows it to record user.

#### Input

||||
|---|---|---|
| `token` | `String` | Application token that you can get in your dashboard |
| `endpoint_url` | `String` | Url where you will be sending data |

#### Example
```rust
use blotout::bo_init;

const TOKEN: &str = "7T3VGKRTMZND4Q9";
const ENDPOINT_URL: &str = "https://domain.com/sdk";

bo_init(TOKEN.to_string(), ENDPOINT_URL.to_string()).await;
```

## bo_capture
The `bo_capture` method is used to record developer events. This allows you to send custom events to the server when a user is interacting with the app. For example, one custom event would be when a user adds an item to a cart.

#### Input

||||
|---|---|---|
| `eventName` | `String` | Name of the event that you are sending. |
| `event_data` | `String` | You can provide some additional data to this event. There is no limitation as this is just a key-value pair send to the server. |
| `screen_name` | `String` | Current screen name that user is on, this will be used for navigation charts for example. |

#### Example
```rust
use blotout::bo_capture;

let event_name = "add_to_cart";
let data = r#"{
    "itemName": "phone"
}"#;
let screen_name = "category_list";

bo_capture(event_name.to_string(), data.to_string(), screen_name.to_string()).await;
```

## bo_capture_personal
Same as the method above (`bo_capture`) `bo_capture_personal` is used to record developer events. The main difference is that `bo_capture_personal` should be used when you are sending personal information to the server. 
This payload will be encrypted on the client-side so that no personal data can be seen while going to the server or even on the server without appropriate permissions.

#### Input

||||
|---|---|---|
| `eventName` | `String` | Name of the event that you are sending. |
| `event_data` | `String` | You can provide some additional data to this event. There is no limitation as this is just a key-value pair send to the server. |
| `is_phi` | `Boolean` | Define if data that you are sending is protected health information (PHI). If this is not defined or is set to false, data is treated as personally identifiable information (PII). |
| `screen_name` | `String`| Current screen name that user is on, this will be used for navigation charts for example. |

```rust
use blotout::bo_capture_personal;

let event_name = "registration";
let data = r#"{
    "email id": "user@example.com",
    "gender": "female"
}"#;
let screen_name = "signup";

bo_capture_personal(event_name.to_string(), data.to_string(), false, screen_name.to_string()).await;
```

## bo_map_id
The `bo_map_id` method allows you to map external services to Blotout ID.

#### Input

||||
|---|---|---|
| `external_id` | `String` | External ID that you want to link to Blotout ID |
| `provider` | `String` | Provider that generated external ID, for example `hubspot` |
| `event_data` | `String` | You can provide some additional data to this event. There is no limitation as this is just a key-value pair send to the server. |

#### Example
```rust
use blotout::bo_map_id;

let external_id = "92j2jr230r-232j9j2342j3-jiji";
let provider = "hubspot";
let data = r#"{
    "lang": "en"
}"#;

bo_map_id(external_id.to_string(), provider.to_string(), data.to_string()).await;
```

## bo_enable_log
The `bo_enable_log` method allows you to print all SDK logs on console.

#### Input

||||
|---|---|---|
| `enable` | `Boolean` | Enable or disable logs, which can help you debug problems. |

#### Example
```rust
use blotout::bo_enable_log;

bo_enable_log(true);
```

## bo_get_user_id
The `bo_get_user_id` method allows you to go get Blotout user id that is linked to all data that is sent to the server.

#### Output
Returns user ID as `String`.

#### Example
```rust
use blotout::bo_get_user_id;

println!(bo_get_user_id());
```

## bo_enable
The `bo_enable` method allows you to enable/disable the sending of analytics data. Enabled by default.

#### Input

||||
|---|---|---|
| `enable` | `Boolean` | Enable or disable SDK |

#### Example
```rust
use blotout::bo_enable;

bo_enable(true);
```
