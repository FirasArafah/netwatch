use sysinfo::Networks;

fn main() {
    let networks = Networks::new_with_refreshed_list();

    if networks.is_empty() {
        eprintln!("No network interfaces found.");
        std::process::exit(1);
    }

    println!("Discovered {} interface(s):\n", networks.len());
    println!("{:<20} {:<20} {:<8}", "NAME", "MAC", "STATE");
    println!("{}", "-".repeat(50));

    for (name, data) in &networks {
        let mac = data.mac_address().to_string();

        // sysinfo has no direct up/down flag; traffic history is a
        // reasonable proxy for now and will be refined in commit 2.
        let state = if data.total_received() > 0 || data.total_transmitted() > 0 {
            "up"
        } else {
            "idle"
        };

        println!("{:<20} {:<20} {:<8}", name, mac, state);
    }
}
