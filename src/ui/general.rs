use crate::core::model::RedKeySnapshot;

pub fn render(ui: &mut egui::Ui, snap: &RedKeySnapshot) {
    ui.heading("General Info");
    ui.separator();

    ui.label(format!("Hostname: {}", snap.general.hostname));
    ui.label(format!(
        "OS: {} {}",
        snap.general.os_name, snap.general.os_version
    ));
}
