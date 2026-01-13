use crate::core::model::{NetInterface, NetworkInfo};
use sysinfo::Networks;

fn macaddr_to_option_string(mac: sysinfo::MacAddr) -> Option<String> {
    // In sysinfo spesso "00:00:00:00:00:00" significa non disponibile
    let bytes = mac.0;
    let all_zero = bytes.iter().all(|&b| b == 0);
    if all_zero {
        None
    } else {
        Some(format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
        ))
    }
}

pub fn collect() -> NetworkInfo {
    let networks = Networks::new_with_refreshed_list();

    let mut interfaces = Vec::new();

    for (name, data) in &networks {
        let mac = macaddr_to_option_string(data.mac_address());

        let ips = data
            .ip_networks()
            .iter()
            .map(|ip| ip.to_string())
            .collect::<Vec<_>>();

        interfaces.push(NetInterface {
            name: name.to_string(),
            mac,
            ips,
        });
    }

    interfaces.sort_by(|a, b| a.name.cmp(&b.name));

    NetworkInfo { interfaces }
}
