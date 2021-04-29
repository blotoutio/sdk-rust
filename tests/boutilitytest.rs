use blotout::utility::common_utility::{
    code_for_codified_event, generate_user_id, get_hash_int_sum, get_sha1_hex, get_timezone_offset,
};
use blotout::utility::file_manager::BOSHAREDFILEINSTANCE;
use blotout::utility::shared_manager::BOSHAREDINSTANCE;
use blotout::utility::system_info_manager::BOSYSTEMINFOINSTANCE;
use serde_json::json;

#[test]
pub fn test_check_unique_id() {
    let user_id = BOSHAREDFILEINSTANCE.lock().unwrap().get_user_id();
    assert!(!user_id.is_empty());
}

#[test]
pub fn test_check_file_permission() {
    let file_permission = BOSHAREDFILEINSTANCE.lock().unwrap().check_file_permission();
    assert!(file_permission);
}

#[test]
pub fn test_path_exists() {
    let file_exist = BOSHAREDFILEINSTANCE
        .lock()
        .unwrap()
        .path_exists("BOAnalyticsRootUserId.txt");
    assert!(file_exist);
}

#[test]
pub fn test_generate_user_id() {
    let user_id = generate_user_id();
    assert!(!user_id.is_empty());
}

#[test]
pub fn test_code_for_codified_event() {
    let event_name_with_underscore = code_for_codified_event("awesome_event".to_string());
    assert_eq!(event_name_with_underscore, 24008);

    let event_name_with_space = code_for_codified_event("some awesome event".to_string());
    assert_eq!(24016, event_name_with_space);

    let event_name_with_ascii = code_for_codified_event("目_awesome_event".to_string());
    assert_eq!(24049, event_name_with_ascii);
}

#[test]
pub fn test_hash_int_sum() {
    let hash_sum = get_hash_int_sum("ab".to_string());
    assert_eq!(2879, hash_sum);
}

#[test]
pub fn test_get_sha1_hex() {
    let sha1_hex = get_sha1_hex("Blotout Inc".to_string());
    assert!(!sha1_hex.is_empty());
}

#[test]
pub fn test_rsa_encryption() {
    let private_key = "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCtfDKGDkF6Da5wvyA53G9naA3POeSrKSsi/AIAISLhKDCBXzXe7MQsoW7IAEqFuDh2578BdzuVFDO/b5q8af4u+GSBIarGM75/biUIV6PcrteywsbgOVsrs5NYgHRoojG283V/f2+aRDN0p30YrlI0msT4epnNbkczIFCoXqK2YQIDAQAB";

    let uuid: String = generate_user_id();

    let encrypted_rsa_key = encrypt_key_using_rsa(uuid, private_key.to_string());

    assert!(!encrypted_rsa_key.is_empty())
}

#[test]

pub fn test_aes_encryption() {
    let mut data =
        "{\"some property\": \"some value\", \"some other property\": \"some other value\"}"
            .to_string();
    data = json!(data).to_string();

    //AES data encryption
    let uuid: String = generate_user_id();
    let iv_string = "Q0BG17E2819IWZYQ";
    let encrypted_data =
        encrypt_data_using_aes(data.as_bytes(), uuid.as_bytes(), iv_string.as_bytes());
    let encrypted_string = base64::encode(encrypted_data.unwrap());
    assert!(!encrypted_string.is_empty())
}

#[test]
pub fn test_aes_decryption() {
    let message = "Hello World!";

    //AES data encryption
    let uuid: String = generate_user_id();
    let iv_string = "Q0BG17E2819IWZYQ";

    let encrypted_data =
        encrypt_data_using_aes(message.as_bytes(), uuid.as_bytes(), iv_string.as_bytes())
            .ok()
            .unwrap();

    let decrypted_data =
        decrypt_data_using_aes(&encrypted_data[..], uuid.as_bytes(), iv_string.as_bytes())
            .ok()
            .unwrap();

    assert!(message.as_bytes() == &decrypted_data[..]);
}

#[test]
pub fn test_system_info() {
    BOSYSTEMINFOINSTANCE.lock().unwrap().init_system_info();
    assert_eq!(
        BOSYSTEMINFOINSTANCE.lock().unwrap().os_type,
        "MacOS".to_string()
    );
    assert_eq!(BOSYSTEMINFOINSTANCE.lock().unwrap().platform_code, 27);
}

#[test]
pub fn test_shared_manager() {
    BOSHAREDINSTANCE.lock().unwrap().set_sdk_enabled(true);
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_token("token".to_string());
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_base_url("http://blotout.io".to_string());
    BOSHAREDINSTANCE.lock().unwrap().set_log_enabled(true);
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_user_id("user_id".to_string());
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_session_id("session_id".to_string());

    assert_eq!(BOSHAREDINSTANCE.lock().unwrap().sdk_enabled, true);
    assert_eq!(BOSHAREDINSTANCE.lock().unwrap().token, "token".to_string());
    assert_eq!(
        BOSHAREDINSTANCE.lock().unwrap().base_url,
        "http://blotout.io".to_string()
    );
    assert_eq!(BOSHAREDINSTANCE.lock().unwrap().log_enabled, true);
    assert_eq!(
        BOSHAREDINSTANCE.lock().unwrap().user_id,
        "user_id".to_string()
    );
    assert_eq!(
        BOSHAREDINSTANCE.lock().unwrap().session_id,
        "session_id".to_string()
    );
}

#[test]
fn test_get_timezone_offset() {
    let timezone_offset = get_timezone_offset();
    assert_eq!(timezone_offset, 330);
}
