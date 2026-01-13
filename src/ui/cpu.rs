use crate::core::model::RedKeySnapshot;

pub fn render(ui: &mut egui::Ui, snap: &RedKeySnapshot) {
    ui.heading("CPU");
    ui.separator();

    ui.label(format!("Brand: {}", snap.cpu.brand));
    ui.label(format!("Cores: {}", snap.cpu.cores));
    ui.label(format!("Frequency: {} MHz", snap.cpu.frequency_mhz));
}
