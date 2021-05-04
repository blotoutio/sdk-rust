use blotout::bo_log_event;
use blotout::bo_log_phi_event;
use blotout::bo_log_pii_event;
use blotout::bo_sdk_init;

const TOKEN: &str = "JB8RWCP66GFTH98"; // Application key
const END_POINT: &str = "https://staging.foobar.com/sdk"; // <1P Container Domain>
const BUNDLE_ID: &str = "com.domain.fooBar"; // Application Unique Bundle ID

#[tokio::main]
async fn main() {
   /*
      Initialize sdk with data that we generated in
      Application section on Blotout Dashboard
    */
    bo_sdk_init(
        TOKEN.to_string(),
        END_POINT.to_string(),
        BUNDLE_ID.to_string(),
    )
    .await;

    /*
       Logging custom events that helps us understand
       what user is doing the application
    */
    let mut event_name = "application_started";
    let mut data = r#"{
        "someProperty": "some value"
    }"#;
    bo_log_event(event_name.to_string(), data.to_string()).await;

    /*
       When user register or fill our form it's good to log this data
       as it will help us connect analytics data. This events are called PII events.
       PII - Personal Identifiable Information
    */
    event_name = "registration";
    data = r#"{
        "email id": "user@example.com",
        "gender": "female"
    }"#;
    bo_log_pii_event(event_name.to_string(), data.to_string()).await;

    /*
       If user fills out any data that is related to health and you want to log it,
       you should use PHI events. PHI events are handled with different policy
       than PII events.
       PHI - Personal Health Information
    */
    event_name = "blood_group";
    data = r#"{
        "email id": "user@example.com",
        "bloodGroup": "A+ve"
    }"#;
    bo_log_phi_event(event_name.to_string(), data.to_string()).await;
}
