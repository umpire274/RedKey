use crate::core::model::RedKeySnapshot;

pub fn render(ui: &mut egui::Ui, snap: &RedKeySnapshot) {
    ui.heading("Network");
    ui.separator();

    if snap.network.interfaces.is_empty() {
        ui.label("No interfaces detected.");
        return;
    }

    for itf in &snap.network.interfaces {
        ui.group(|ui| {
            ui.label(format!("Interface: {}", itf.name));
            if let Some(mac) = &itf.mac {
                ui.label(format!("MAC: {}", mac));
            }
            if itf.ips.is_empty() {
                ui.label("IPs: (none)");
            } else {
                ui.label("IPs:");
                for ip in &itf.ips {
                    ui.label(format!("  - {}", ip));
                }
            }
        });
        ui.add_space(8.0);
    }
}
