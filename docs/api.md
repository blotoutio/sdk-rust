# API

## init
The `bo_init` method is used for initializing SDK. This sets all required configurations and also sends system event `sdk_start` which allows it to record user.

#### Input
`pub async fn bo_init(token: String, endpoint_url: String) -> bool`

|||||
|---|---|---|---|
| `token` | `String` | Application token that you can get in your dashboard |
| `endpoint_url` | `String` | Url where you will be sending data |

#### Example
```rust
bo_init(
        TOKEN.to_string(),
        END_POINT.to_string(),
    ).await;
```

## capture events
The `bo_log_event` method is used to record developer events. This allows you to send custom events to the server when a user is interacting with the app. For example, one custom event would be when a user adds an item to a cart.

## Non-Timed Events
Non-Timed events are generally events which are not time bound and do not contain duration information. For example, the Home Page loaded is non-timed but Home page loading started and home page loading ended, when grouped together, can be a timed event.
These events are categorized under two main categories in Blotout’s SDK.

1: Developer Events:
Developer Events are those which developers codify in the Application code with the help of Blotout’s SDK and SDK sync with Blotout’s server, like “iPhone added to cart“.

```html
pub async fn bo_log_event(event_name: String, data: String) -> bool
```

#### Example
```rust
let mut event_name = "application_started";
let mut data = r#"{
    "someProperty": "some value"
}"#;
bo_log_event(event_name.to_string(), data.to_string()).await;
```

## PII & PHI Events
PII (Personal Identifiable Information) events are like developer codified events that carry sensitive information related to User.
PHI ( Protected Health information) events are like PII but carries user’s private health information
In Blotout managed or deployed Infrastructure, PII and PHI events data is encrypted using asymmetric encryption algorithms and provides access to authenticated users only.
Below methods can be used to log PII and PHI information.

```html

pub async fn bo_log_pii_event(event_name: String, data: String) -> bool

pub async fn bo_log_phi_event(event_name: String, data: String) -> bool

```

|||||
|---|---|---|---|
| `event_name` | `String` |  | Name of the event that you are sending |
| `data` | `Object` | Optional | You can provide some additional data to this event. There is no limitation as this is just a key-value pair send to the server. |


#### Example

```rust
event_name = "registration";
data = r#"{
    "email id": "user@example.com",
    "gender": "female"
}"#;
bo_log_pii_event(event_name.to_string(), data.to_string()).await;
```

## mapID
The `bo_map_id` method allows you to map external services to Blotout ID.

#### Input
`pub async fn bo_map_id(externalId: String, provider: String, data: String) -> bool`

|||||
|---|---|---|---|
| `external_id` | `String` |  | External ID that you want to link to Blotout ID |
| `provider` | `String` |  | Provider that generated external ID, for example `hubspot` |
| `data` | `Object` | Optional | You can provide some additional data to this event. There is no limitation as this is just a key-value pair send to the server. |

#### Example
```rust
external_id = "92j2jr230r-232j9j2342j3-jiji";
provider = "hubspot";
data = r#"{
    "email id": "user@example.com",
    "gender": "female"
}"#;
bo_map_id(external_id.to_string(), provider.to_string(), data.to_string()).await;
```
