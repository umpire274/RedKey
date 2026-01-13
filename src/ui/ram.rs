use crate::core::model::RedKeySnapshot;

pub fn render(ui: &mut egui::Ui, snap: &RedKeySnapshot) {
    ui.heading("RAM");
    ui.separator();

    ui.label(format!("Total: {} MB", snap.ram.total_bytes / 1024 / 1024));
    ui.label(format!("Used:  {} MB", snap.ram.used_bytes / 1024 / 1024));
}
