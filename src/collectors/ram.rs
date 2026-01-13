use crate::core::model::RamInfo;
use sysinfo::System;

pub fn collect() -> RamInfo {
    let mut sys = System::new();
    sys.refresh_memory();

    // sysinfo usa KiB
    let total_bytes = sys.total_memory().saturating_mul(1024);
    let used_bytes = sys.used_memory().saturating_mul(1024);

    RamInfo {
        total_bytes,
        used_bytes,
    }
}
