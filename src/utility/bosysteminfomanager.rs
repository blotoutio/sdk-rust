extern crate sys_info;

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;
use sys_info::*;

#[derive(Default)]
pub struct BOSystemInfoManager {
    pub os_type: String,
    pub platform_code: i64,
}

lazy_static! {
    pub static ref BOSYSTEMINFOINSTANCE: Mutex<BOSystemInfoManager> =
        Mutex::new(BOSystemInfoManager::default());
}

impl BOSystemInfoManager {
    pub fn init_system_info(&mut self) {
        self.os_type = os_type().unwrap();

        if self.os_type.eq("Darwin") {
            self.platform_code = 27;
            self.os_type = "MacOS".to_string();
        } else if self.os_type.eq("Linux") {
            self.platform_code = 28;
        } else if self.os_type.eq("Windows") {
            self.platform_code = 26;
        } else {
            self.platform_code = 80;
        }
    }
}
