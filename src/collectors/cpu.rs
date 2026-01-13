use crate::core::model::CpuInfo;
use sysinfo::System;

pub fn collect() -> CpuInfo {
    let mut sys = System::new();
    sys.refresh_cpu_all();

    let cpus = sys.cpus();
    let cores = cpus.len();

    let brand = cpus
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let frequency_mhz = cpus.first().map(|c| c.frequency()).unwrap_or(0);

    CpuInfo {
        brand,
        cores,
        frequency_mhz,
    }
}
