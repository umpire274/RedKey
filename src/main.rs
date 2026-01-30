mod app;
mod collectors;
mod core;
mod ui;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../res/redkey_256.png"))
        .expect("Failed to load icon");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(icon)
            .with_title("RedKey")
            .with_inner_size([1000.0, 650.0]),
        ..Default::default()
    };

    if let Err(e) = eframe::run_native(
        "RedKey",
        native_options,
        Box::new(|cc| Ok(Box::new(app::RedKeyApp::new(cc)))),
    ) {
        // eframe::Error non è Send/Sync -> lo convertiamo in messaggio
        return Err(anyhow::anyhow!(e.to_string()));
    }

    Ok(())
}
