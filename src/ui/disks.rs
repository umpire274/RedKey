use crate::core::model::RedKeySnapshot;

pub fn render(ui: &mut egui::Ui, snap: &RedKeySnapshot) {
    ui.heading("Disks");
    ui.separator();

    if snap.disks.disks.is_empty() {
        ui.label("No disks detected.");
        return;
    }

    for d in &snap.disks.disks {
        ui.group(|ui| {
            ui.label(format!("Name: {}", d.name));
            ui.label(format!("Mount: {}", d.mount));
            if let Some(fs) = &d.file_system {
                ui.label(format!("FS: {}", fs));
            }
            ui.label(format!("Total: {} GB", d.total_bytes / 1024 / 1024 / 1024));
            ui.label(format!(
                "Available: {} GB",
                d.available_bytes / 1024 / 1024 / 1024
            ));
        });
        ui.add_space(8.0);
    }
}
