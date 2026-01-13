use crate::core::model::GeneralInfo;
use sysinfo::System;

pub fn collect() -> GeneralInfo {
    let mut sys = System::new();
    sys.refresh_all();

    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());

    GeneralInfo {
        os_name,
        os_version,
        hostname,
    }
}
