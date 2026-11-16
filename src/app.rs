use crate::format::{format_bytes, format_rate};
use eframe::egui;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use sysinfo::Networks;

pub struct NetwatchApp {
    networks: Networks,
    prev_traffic: HashMap<String, (u64, u64)>,
    last_tick: Instant,
}

impl NetwatchApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Apply a professional dark theme
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(20, 20, 24);
        cc.egui_ctx.set_visuals(visuals);

        let networks = Networks::new_with_refreshed_list();

        // Snapshot initial traffic so the first delta is accurate
        let prev_traffic = networks
            .iter()
            .map(|(name, data)| {
                (
                    name.clone(),
                    (data.total_received(), data.total_transmitted()),
                )
            })
            .collect();

        Self {
            networks,
            prev_traffic,
            last_tick: Instant::now(),
        }
    }
}

impl eframe::App for NetwatchApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 1. Tell egui to repaint this window in exactly 1 second
        ui.ctx().request_repaint_after(Duration::from_secs(1));

        // 2. Check if 1 second has actually passed since the last frame
        let elapsed = self.last_tick.elapsed().as_secs_f64().max(0.001);

        if elapsed >= 1.0 {
            // Refresh system data
            self.networks.refresh();
            self.last_tick = Instant::now();
        }

        // Top Navigation Bar
        ui.horizontal(|ui| {
            let _ = ui.selectable_label(true, "📊 Dashboard");
            let _ = ui.selectable_label(false, "🛡️ Packet Capture");
            let _ = ui.selectable_label(false, "⚙️ Settings");
        });

        ui.separator();
        ui.add_space(10.0);

        // Header
        ui.horizontal(|ui| {
            ui.heading(egui::RichText::new("Network Interfaces").color(egui::Color32::WHITE));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new(format!("{} interfaces", self.networks.len())).weak());
            });
        });
        ui.add_space(10.0);

        // 3. The Grid with Live Data
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("interface_grid")
                .num_columns(5) // Added 2 columns for RX/s and TX/s
                .spacing([30.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    // Table Headers
                    ui.label(
                        egui::RichText::new("STATUS")
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::GRAY),
                    );
                    ui.label(
                        egui::RichText::new("NAME")
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::GRAY),
                    );
                    ui.label(
                        egui::RichText::new("MAC ADDRESS")
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::GRAY),
                    );
                    ui.label(
                        egui::RichText::new("RX/s")
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::GRAY),
                    );
                    ui.label(
                        egui::RichText::new("TX/s")
                            .strong()
                            .size(12.0)
                            .color(egui::Color32::GRAY),
                    );
                    ui.end_row();

                    // Table Data
                    for (name, data) in &self.networks {
                        // Get current totals
                        let total_rx = data.total_received();
                        let total_tx = data.total_transmitted();

                        // Calculate deltas (rates)
                        let (prev_rx, prev_tx) =
                            self.prev_traffic.get(name).copied().unwrap_or((0, 0));
                        let rx_rate = (total_rx.saturating_sub(prev_rx) as f64 / elapsed) as u64;
                        let tx_rate = (total_tx.saturating_sub(prev_tx) as f64 / elapsed) as u64;

                        // Status Column
                        let is_up = total_rx > 0 || total_tx > 0;
                        if is_up {
                            ui.label(
                                egui::RichText::new("🟢 Up")
                                    .color(egui::Color32::from_rgb(80, 200, 120)),
                            );
                        } else {
                            ui.label(egui::RichText::new("⚪ Idle").color(egui::Color32::GRAY));
                        }

                        // Name Column
                        ui.label(egui::RichText::new(name).color(egui::Color32::WHITE));

                        // MAC Address Column
                        let mac = data.mac_address().to_string();
                        ui.label(
                            egui::RichText::new(mac)
                                .monospace()
                                .color(egui::Color32::from_rgb(150, 150, 160)),
                        );

                        // Live RX/s Column
                        ui.label(
                            egui::RichText::new(format_rate(rx_rate))
                                .monospace()
                                .color(egui::Color32::from_rgb(100, 200, 255)),
                        ); // Blue

                        // Live TX/s Column
                        ui.label(
                            egui::RichText::new(format_rate(tx_rate))
                                .monospace()
                                .color(egui::Color32::from_rgb(255, 180, 100)),
                        ); // Orange

                        ui.end_row();
                    }
                });
        });

        // 4. Update the snapshot for the NEXT frame
        // We do this at the end of the UI cycle to ensure we captured the deltas correctly.
        for (name, data) in &self.networks {
            self.prev_traffic.insert(
                name.clone(),
                (data.total_received(), data.total_transmitted()),
            );
        }
    }
}
