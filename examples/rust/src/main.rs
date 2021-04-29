use blotout::bo_log_event;
use blotout::bo_log_phi_event;
use blotout::bo_log_pii_event;
use blotout::bo_init;
use blotout::bo_log_enabled;

const TOKEN: &str = "7T3VGKRTMZND4Q9"; // Application key
const END_POINT: &str = "https://stage.blotout.io/sdk"; // <1P Container Domain>

#[tokio::main]
async fn main() {
  /*
    Enable logging
   */
  bo_log_enabled(true);

   /*
      Initialize sdk with data that we generated in
      Application section on Blotout Dashboard
    */
    bo_init(
        TOKEN.to_string(),
        END_POINT.to_string(),
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
