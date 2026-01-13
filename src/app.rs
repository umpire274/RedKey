use crate::core::{model::RedKeySnapshot, refresh::RefreshController};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    General,
    Cpu,
    Ram,
    Network,
    Disks,
}

pub struct RedKeyApp {
    tab: Tab,
    snapshot: RedKeySnapshot,
    refresh: RefreshController,
}

impl RedKeyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut refresh = RefreshController::new();
        let snapshot = refresh.refresh_all();

        Self {
            tab: Tab::General,
            snapshot,
            refresh,
        }
    }
}

impl eframe::App for RedKeyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top bar: titolo + refresh
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("RedKey");
                ui.separator();

                if ui.button("Refresh").clicked() {
                    self.snapshot = self.refresh.refresh_all();
                }

                ui.separator();
                ui.small(format!("Last refresh: {}", self.snapshot.meta.last_refresh));
            });
        });

        // Bottom tab bar
        egui::TopBottomPanel::bottom("bottom_tabs").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                tab_button(ui, &mut self.tab, Tab::General, "General Info");
                tab_button(ui, &mut self.tab, Tab::Cpu, "CPU");
                tab_button(ui, &mut self.tab, Tab::Ram, "RAM");
                tab_button(ui, &mut self.tab, Tab::Network, "Network");
                tab_button(ui, &mut self.tab, Tab::Disks, "Disks");
            });
        });

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| match self.tab {
            Tab::General => crate::ui::general::render(ui, &self.snapshot),
            Tab::Cpu => crate::ui::cpu::render(ui, &self.snapshot),
            Tab::Ram => crate::ui::ram::render(ui, &self.snapshot),
            Tab::Network => crate::ui::network::render(ui, &self.snapshot),
            Tab::Disks => crate::ui::disks::render(ui, &self.snapshot),
        });
    }
}

fn tab_button(ui: &mut egui::Ui, tab: &mut Tab, value: Tab, label: &str) {
    let selected = *tab == value;
    if ui.selectable_label(selected, label).clicked() {
        *tab = value;
    }
}
