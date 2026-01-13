use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct MetaInfo {
    pub last_refresh: DateTime<Local>,
}

#[derive(Debug, Clone, Default)]
pub struct CpuInfo {
    pub brand: String,
    pub cores: usize,
    pub frequency_mhz: u64,
}

#[derive(Debug, Clone, Default)]
pub struct RamInfo {
    pub total_bytes: u64,
    pub used_bytes: u64,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkInfo {
    pub interfaces: Vec<NetInterface>,
}

#[derive(Debug, Clone, Default)]
pub struct NetInterface {
    pub name: String,
    pub mac: Option<String>,
    pub ips: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DisksInfo {
    pub disks: Vec<DiskItem>,
}

#[derive(Debug, Clone, Default)]
pub struct DiskItem {
    pub name: String,
    pub mount: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub file_system: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct GeneralInfo {
    pub os_name: String,
    pub os_version: String,
    pub hostname: String,
}

#[derive(Debug, Clone)]
pub struct RedKeySnapshot {
    pub meta: MetaInfo,
    pub general: GeneralInfo,
    pub cpu: CpuInfo,
    pub ram: RamInfo,
    pub network: NetworkInfo,
    pub disks: DisksInfo,
}

impl Default for RedKeySnapshot {
    fn default() -> Self {
        Self {
            meta: MetaInfo {
                last_refresh: Local::now(),
            },
            general: GeneralInfo::default(),
            cpu: CpuInfo::default(),
            ram: RamInfo::default(),
            network: NetworkInfo::default(),
            disks: DisksInfo::default(),
        }
    }
}
