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
    //initialise system information
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

        // println!("os: {} {}", os_type().unwrap(), os_release().unwrap());
        // println!("cpu: {} cores, {} MHz", cpu_num().unwrap(), cpu_speed().unwrap());
        // println!("proc total: {}", proc_total().unwrap());
        // let load = loadavg().unwrap();
        // println!("load: {} {} {}", load.one, load.five, load.fifteen);
        // let mem = mem_info().unwrap();
        // println!("mem: total {} KB, free {} KB, avail {} KB, buffers {} KB, cached {} KB",
        //         mem.total, mem.free, mem.avail, mem.buffers, mem.cached);
        // println!("swap: total {} KB, free {} KB", mem.swap_total, mem.swap_free);
        // #[cfg(not(target_os = "solaris"))] {
        //     let disk = disk_info().unwrap();
        //     println!("disk: total {} KB, free {} KB", disk.total, disk.free);
        // }
        // println!("hostname: {}", hostname().unwrap());
        // #[cfg(not(target_os = "windows"))] {
        //     let t = boottime().unwrap();
        //     println!("boottime {} sec, {} usec", t.tv_sec, t.tv_usec);
        // }
    }
}
