use crate::utility::common_utility::BOSHAREDCOMMONUTILITYINSTANCE;
use crate::utility::shared_manager::BOSHAREDINSTANCE;
use lazy_static::lazy_static; // 1.4.0
use rand::prelude::*;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Mutex;

const BO_ANALYTICS_ROOT_USER_ID: &str = "BOAnalyticsRootUserId.txt";
const BO_ANALYTICS_ROOT_TEST: &str = "BOAnalyticsRootTest.txt";

#[derive(Default)]
pub struct BoFileManager {
    pub user_id: String,
}

lazy_static! {
    pub static ref BOSHAREDFILEINSTANCE: Mutex<BoFileManager> =
        Mutex::new(BoFileManager::default());
}

impl BoFileManager {
    pub fn check_file_permission(&self) -> bool {
        let path = Path::new(BO_ANALYTICS_ROOT_TEST);

        let file = File::create(&path);

        if file.is_ok() {
            //delete the file
            let response = fs::remove_file(path);
            if BOSHAREDINSTANCE.lock().unwrap().log_enabled {
                println!("File IO Error : {:?}", response.is_ok());
            }

            true
        } else {
            false
        }
    }

    pub fn get_unique_device_id(&self) -> String {
        let user_id = BOSHAREDCOMMONUTILITYINSTANCE
            .lock()
            .unwrap()
            .get_device_id(); //self.generate_user_id();
        user_id
    }
    //generate and save a unique userid into file
    pub fn save_user_id(&self) -> String {
        if !self.path_exists(BO_ANALYTICS_ROOT_USER_ID) {
            //create file
            let path = Path::new(BO_ANALYTICS_ROOT_USER_ID);

            let file = File::create(&path);
            let mut file = match file {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            };
            let user_id = self.get_unique_device_id(); //self.generate_user_id();
            let result = file.write_all(user_id.as_bytes());
            if result.is_ok() {
                BOSHAREDINSTANCE
                    .lock()
                    .unwrap()
                    .set_user_id(user_id.to_owned());
                return user_id;
            } else {
                return BOSHAREDINSTANCE.lock().unwrap().user_id.to_owned();
            }
        }

        "".parse().unwrap()
    }

    //check and return a unique user id from file storage
    pub fn get_user_id(&self) -> String {
        let user_id = BOSHAREDINSTANCE.lock().unwrap().user_id.to_owned();
        if !user_id.is_empty() {
            user_id
        } else if self.path_exists(BO_ANALYTICS_ROOT_USER_ID) {
            let file = File::open(BO_ANALYTICS_ROOT_USER_ID);

            let mut file = match file {
                Ok(file) => file,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            };

            let mut contents = String::new();
            let result = file.read_to_string(&mut contents);
            if result.is_ok() {
                contents
            } else {
                "".parse().unwrap()
            }
        } else {
            self.save_user_id()
        }
    }

    pub fn generate_user_id(&self) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        abcdefghijklmnopqrstuvwxyz\
        0123456789)(*&^%$#@!~";
        const USER_ID_LEN: usize = 32;
        let mut rng = rand::thread_rng();

        let user_id: String = (0..USER_ID_LEN)
            .map(|_| {
                let idx = rng.gen_range(0, CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        user_id
    }

    //check if file path exists
    pub fn path_exists(&self, path: &str) -> bool {
        fs::metadata(path).is_ok()
    }
}
