//! netwatch — a Rust-based network monitor for security analysis.
//!
//! Milestone 2: live RX/TX rates refreshed once per second.
//!
//! This is a simple network monitor that displays the current RX and TX rates
//! for each network interface, along with the total amount of data received
//! and transmitted.
//!
//! Requires an ANSI-capable terminal (Windows Terminal, or cmd.exe on
//! Windows 10 1511+). Run with `cargo run` and stop with Ctrl+C.

mod format;

use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

use sysinfo::Networks;

use crate::format::{format_bytes, format_rate};

fn main() {
    let mut networks = Networks::new_with_refreshed_list();

    if networks.is_empty() {
        eprintln!("No network interfaces found.");
        std::process::exit(1);
    }

    // Snapshot totals so the first delta is meaningful.
    let mut prev: HashMap<String, (u64, u64)> = networks
        .iter()
        .map(|(name, data)| {
            (
                name.clone(),
                (data.total_received(), data.total_transmitted()),
            )
        })
        .collect();

    let mut last_tick = Instant::now();

    loop {
        thread::sleep(Duration::from_secs(1));
        networks.refresh();

        // Measure actual elapsed time so rates stay accurate even if
        // `refresh()` itself is slow on a machine with many interfaces.
        let now = Instant::now();
        let elapsed = now.duration_since(last_tick).as_secs_f64().max(0.001);
        last_tick = now;

        // Clear the screen and home the cursor (ANSI).
        print!("\x1b[2J\x1b[H");
        println!("netwatch — live interface stats (refresh: 1s, Ctrl+C to stop)\n");

        println!(
            "{:<40} {:>14} {:>14} {:>16} {:>16}",
            "NAME", "RX/s", "TX/s", "RX total", "TX total"
        );
        println!("{}", "-".repeat(104));

        for (name, data) in &networks {
            let total_rx = data.total_received();
            let total_tx = data.total_transmitted();

            let (prev_rx, prev_tx) = prev.get(name).copied().unwrap_or((0, 0));

            // `saturating_sub` guards against counter resets when an
            // interface goes down and comes back up.
            let rx_rate = (total_rx.saturating_sub(prev_rx) as f64 / elapsed) as u64;
            let tx_rate = (total_tx.saturating_sub(prev_tx) as f64 / elapsed) as u64;

            prev.insert(name.clone(), (total_rx, total_tx));

            println!(
                "{:<40} {:>14} {:>14} {:>16} {:>16}",
                truncate(name, 40),
                format_rate(rx_rate),
                format_rate(tx_rate),
                format_bytes(total_rx),
                format_bytes(total_tx),
            );
        }
    }
}

/// Truncates `s` to at most `max` chars, appending `…` if cut.
/// Char-based, so multibyte UTF-8 names don't get split mid-codepoint.
fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
        out.push('…');
        out
    }
}
