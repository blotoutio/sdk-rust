use blotout::{bo_init, bo_capture, bo_capture_personal, bo_map_id, bo_enable_log, bo_get_user_id};

const TOKEN: &str = "7T3VGKRTMZND4Q9"; // Application key
const END_POINT: &str = "https://stage.blotout.io/sdk"; // <1P Container Domain>

#[tokio::main]
async fn main() {
    /*
      Enable logging
     */
    bo_enable_log(true);

    /*
       Initialize sdk with data that we generated in
       Application section on Blotout Dashboard
     */
    bo_init(TOKEN.to_string(), END_POINT.to_string()).await;

    /*
       Logging custom events that helps us understand
       what user is doing the application
    */
    let mut event_name = "add_to_cart".to_string();
    let mut screen_name = "home".to_string();
    let mut data = r#"{
        "item": "phone"
    }"#
    .to_string();
    bo_capture(event_name, data, screen_name).await;

    /*
       When user register or fill our form it's good to log this data
       as it will help us connect analytics data. This events are called PII events.
       PII - Personal Identifiable Information
    */
    event_name = "user_registration".to_string();
    screen_name = "registration".to_string();
    data = r#"{
        "email": "user@example.com",
        "gender": "female"
    }"#
    .to_string();
    bo_capture_personal(event_name, data, false, screen_name).await;

    /*
       If user fills out any data that is related to health and you want to log it,
       you should use PHI events. PHI events are handled with different policy
       than PII events.
       PHI - Personal Health Information
    */
    event_name = "blood_group".to_string();
    screen_name = "signup".to_string();
    data = r#"{
        "bloodGroup": "A+ve"
    }"#
    .to_string();
    bo_capture_personal(event_name, data, true, screen_name).await;

    /*
        Map ID
     */
    let map_id = "2f28023hj0-2323-23232".to_string();
    let map_provider = "service".to_string();
    data = r#"{
        "lang": "en"
    }"#.to_string();
    bo_map_id(map_id, map_provider, data).await;

    /*
        Get user ID
     */
    println!("User ID: {}", bo_get_user_id());
}
