use blotout::utility::bocommonutility::BOSHAREDCOMMONUTILITYINSTANCE;
use blotout::utility::boerror::BOError;
use blotout::utility::bofilemanager::BOSHAREDFILEINSTANCE;
use blotout::utility::bosharedmanager::BOSHAREDINSTANCE;
use blotout::utility::bosysteminfomanager::BOSYSTEMINFOINSTANCE;
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
pub fn test_get_unique_device_id() {
    let device_id = BOSHAREDFILEINSTANCE.lock().unwrap().get_unique_device_id();
    assert!(!device_id.is_empty());
}

#[test]
pub fn test_get_device_id() {
    let device_id = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .get_device_id();
    assert!(!device_id.is_empty());
}

#[test]
pub fn test_generate_user_id() {
    let user_id = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .generate_user_id();
    assert!(!user_id.is_empty());
}

#[test]
pub fn test_guid_string() {
    let device_id = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .get_device_id();
    let guid_string = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .get_guid_str(device_id);
    assert!(!guid_string.is_empty());
}

#[test]
pub fn test_code_for_custom_codified_event() {
    let event_name_with_underscore = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .code_for_custom_codified_event("awesome_event".to_string());
    assert_eq!(event_name_with_underscore, 24008);

    let event_name_with_space = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .code_for_custom_codified_event("some awesome event".to_string());
    assert_eq!(24016, event_name_with_space);

    let event_name_with_ascii = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .code_for_custom_codified_event("目_awesome_event".to_string());
    assert_eq!(24049, event_name_with_ascii);
}

#[test]
pub fn test_hash_int_sum() {
    let hash_sum = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .get_hash_int_sum("ab".to_string());
    assert_eq!(2879, hash_sum);
}

#[test]
pub fn test_get_sha1_hex() {
    let sha1_hex = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .get_sha1_hex("Blotout Inc".to_string());
    assert!(!sha1_hex.is_empty());
}

#[test]
pub fn test_rsa_encryption() {
    let private_key = "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCtfDKGDkF6Da5wvyA53G9naA3POeSrKSsi/AIAISLhKDCBXzXe7MQsoW7IAEqFuDh2578BdzuVFDO/b5q8af4u+GSBIarGM75/biUIV6PcrteywsbgOVsrs5NYgHRoojG283V/f2+aRDN0p30YrlI0msT4epnNbkczIFCoXqK2YQIDAQAB";

    let uuid: String = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .generate_user_id();

    let encrypted_rsa_key = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .encrypt_key_using_rsa(uuid, private_key.to_string());

    assert!(!encrypted_rsa_key.is_empty())
}

#[test]

pub fn test_aes_encryption() {
    let mut data =
        "{\"some property\": \"some value\", \"some other property\": \"some other value\"}"
            .to_string();
    data = json!(data).to_string();

    //AES data encryption
    let uuid: String = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .generate_user_id();
    let iv_string = "Q0BG17E2819IWZYQ";
    let encrypted_data = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .encrypt_data_using_aes(data.as_bytes(), uuid.as_bytes(), iv_string.as_bytes());
    let encrypted_string = base64::encode(encrypted_data.unwrap());
    assert!(!encrypted_string.is_empty())
}

#[test]
pub fn test_aes_decryption() {
    let message = "Hello World!";

    //AES data encryption
    let uuid: String = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .generate_user_id();
    let iv_string = "Q0BG17E2819IWZYQ";

    let encrypted_data = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .encrypt_data_using_aes(message.as_bytes(), uuid.as_bytes(), iv_string.as_bytes())
        .ok()
        .unwrap();

    let decrypted_data = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .decrypt_data_using_aes(&encrypted_data[..], uuid.as_bytes(), iv_string.as_bytes())
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
        .set_bundle_id("bundle_id".to_string());
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
        BOSHAREDINSTANCE.lock().unwrap().bundle_id,
        "bundle_id".to_string()
    );
    assert_eq!(
        BOSHAREDINSTANCE.lock().unwrap().session_id,
        "session_id".to_string()
    );
}

#[test]
fn test_enum() {
    assert!(Some(BOError::ManifestNotAvailable).is_some());
}

#[test]
fn test_get_sub_string() {
    let uuid: String = "Blotout Inc".to_string();
    let sub_string = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .get_sub_string(0, 4, uuid);
    assert_eq!(sub_string, "Blot".to_string());
}
#[test]
fn test_get_timezone_offset() {
    let timezone_offset = BOSHAREDCOMMONUTILITYINSTANCE
        .lock()
        .unwrap()
        .get_timezone_offset();
    assert_eq!(timezone_offset, 330);
}
