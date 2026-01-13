use crate::collectors;
use crate::core::model::RedKeySnapshot;
use chrono::Local;

pub struct RefreshController;

impl RefreshController {
    pub fn new() -> Self {
        Self
    }

    pub fn refresh_all(&mut self) -> RedKeySnapshot {
        let general = collectors::general::collect();
        let cpu = collectors::cpu::collect();
        let ram = collectors::ram::collect();
        let network = collectors::network::collect();
        let disks = collectors::disks::collect();

        RedKeySnapshot {
            meta: crate::core::model::MetaInfo {
                last_refresh: Local::now(),
            },
            general,
            cpu,
            ram,
            network,
            disks,
        }
    }
}
