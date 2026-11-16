//! netwatch — a Rust-based network monitor for security analysis.
//!
//! Milestone 3: egui GUI scaffold.

mod app;
mod format;

use app::NetwatchApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    // Configure the native window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0])
            .with_title("netwatch"),
        ..Default::default()
    };

    // Launch the app
    eframe::run_native(
        "netwatch",
        options,
        Box::new(|cc| Ok(Box::new(NetwatchApp::new(cc)))),
    )
}
