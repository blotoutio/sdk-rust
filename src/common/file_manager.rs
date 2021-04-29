use crate::common::shared_manager::BOSHAREDINSTANCE;
use crate::common::utils::create_user_id;
use crate::model::storage::Storage;

use chrono::Utc;
use serde_json::json;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const FILE_NAME: &str = "BOAnalytics.txt";
const FILE_NAME_TEST: &str = "BOAnalyticsTest.txt";

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn check_file_permission() -> bool {
    let path = Path::new(FILE_NAME_TEST);
    let file = File::create(&path);

    if file.is_ok() {
        let response = fs::remove_file(path);
        if BOSHAREDINSTANCE.lock().unwrap().log_enabled {
            println!("File permission correct: {:?}", response.is_ok());
        }

        true
    } else {
        false
    }
}

fn get_file() -> String {
    if !path_exists(FILE_NAME) {
        return "".to_string();
    }

    let open = File::open(FILE_NAME);
    let mut file = match open {
        Ok(file) => file,
        Err(error) => panic!("Problem creating the file: {:?}", error),
    };

    let mut contents = String::new();
    let result = file.read_to_string(&mut contents);
    if result.is_ok() {
        contents
    } else {
        "".to_string()
    }
}

pub fn load_persisted_data() -> bool {
    if !check_file_permission() {
        return false;
    }

    let contents = get_file();
    let mut data: Storage = Storage {
        user_id: "".to_string(),
        user_id_created: 0,
    };

    if !contents.is_empty() {
        let result = serde_json::from_str(&*contents);
        if result.is_ok() {
            data = result.unwrap()
        }
    }

    if data.user_id.is_empty() {
        data.user_id = create_user_id()
    }

    if data.user_id_created == 0 {
        data.user_id_created = Utc::now().timestamp_millis()
    }

    // save to file
    let data_string = json!(data).to_string();
    let mut file = File::create(FILE_NAME).unwrap();
    let result = file.write_all(data_string.as_bytes());

    if result.is_err() {
        return false;
    }

    // set to shared instance
    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_user_id(data.user_id.to_owned());

    BOSHAREDINSTANCE
        .lock()
        .unwrap()
        .set_user_id_created(data.user_id_created.to_owned());

    true
}
