use chrono::Utc;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use uuid::Uuid;

extern crate crypto;
extern crate rand;

const CODIFIED_EVENT_START: u64 = 21100;

fn get_sha1_hex(input: String) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(input.as_str());
    hasher.result_str()
}

fn get_hash_int_sum(input: String) -> u64 {
    let input_str = input.to_lowercase();
    let sha1_string = get_sha1_hex(input_str);
    let mut sum = 0;
    for char_val in sha1_string.chars() {
        sum += char_val as u64;
    }

    sum
}

pub fn create_user_id() -> String {
    let timestamp = Utc::now().timestamp_millis().to_string();
    format!(
        "{}-{}-{}",
        Uuid::new_v4().to_string(),
        timestamp,
        Uuid::new_v4().to_string()
    )
}

pub fn get_mid(event_name: String) -> String {
    let timestamp = Utc::now().timestamp_millis().to_string();
    format!(
        "{}-{}-{}",
        base64::encode(event_name),
        Uuid::new_v4().to_string(),
        timestamp
    )
}

pub fn code_for_codified_event(event_name: String) -> u64 {
    let event = event_name.trim().to_string();
    let event_name_int_sum = get_hash_int_sum(event);
    let event_name_int_sum_modulo = event_name_int_sum % 8899;
    CODIFIED_EVENT_START + event_name_int_sum_modulo
}

pub fn get_timezone_offset() -> i64 {
    let tz_offset = (chrono::offset::Local::now()
        .date()
        .offset()
        .local_minus_utc()
        / 60) as i64;

    tz_offset
}
