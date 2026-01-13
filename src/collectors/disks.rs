use crate::core::model::{DiskItem, DisksInfo};
use sysinfo::Disks;

pub fn collect() -> DisksInfo {
    let disks = Disks::new_with_refreshed_list();

    let mut out = Vec::new();

    for d in &disks {
        let name = d.name().to_string_lossy().to_string();
        let mount = d.mount_point().to_string_lossy().to_string();
        let total_bytes = d.total_space();
        let available_bytes = d.available_space();

        let fs_str = d.file_system().to_string_lossy().to_string();
        let file_system = if fs_str.is_empty() {
            None
        } else {
            Some(fs_str)
        };

        out.push(DiskItem {
            name,
            mount,
            total_bytes,
            available_bytes,
            file_system,
        });
    }

    out.sort_by(|a, b| a.mount.cmp(&b.mount));

    DisksInfo { disks: out }
}
