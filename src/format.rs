//! Human-readable formatting helpers for network statistics.

/// Formats a byte count into a human-readable string with binary units (IEC).
///
/// Examples:
/// - `format_bytes(0)`         -> `"0 B"`
/// - `format_bytes(1024)`      -> `"1.0 KiB"`
/// - `format_bytes(1536)`      -> `"1.5 KiB"`
/// - `format_bytes(1_048_576)` -> `"1.0 MiB"`
pub fn format_bytes(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    const GIB: f64 = MIB * 1024.0;
    const TIB: f64 = GIB * 1024.0;

    let b = bytes as f64;
    if b < KIB {
        format!("{} B", bytes)
    } else if b < MIB {
        format!("{:.1} KiB", b / KIB)
    } else if b < GIB {
        format!("{:.1} MiB", b / MIB)
    } else if b < TIB {
        format!("{:.1} GiB", b / GIB)
    } else {
        format!("{:.1} TiB", b / TIB)
    }
}

/// Formats a byte rate as `<bytes>/s`, e.g. `format_rate(2048)` -> `"2.0 KiB/s"`.
pub fn format_rate(bytes_per_sec: u64) -> String {
    format!("{}/s", format_bytes(bytes_per_sec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_zero() {
        assert_eq!(format_bytes(0), "0 B");
    }

    #[test]
    fn bytes_small() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1023), "1023 B");
    }

    #[test]
    fn bytes_kib() {
        assert_eq!(format_bytes(1024), "1.0 KiB");
        assert_eq!(format_bytes(1536), "1.5 KiB");
    }

    #[test]
    fn bytes_mib_gib() {
        assert_eq!(format_bytes(1_048_576), "1.0 MiB");
        assert_eq!(format_bytes(1_073_741_824), "1.0 GiB");
    }

    #[test]
    fn rate_appends_suffix() {
        assert_eq!(format_rate(2048), "2.0 KiB/s");
        assert_eq!(format_rate(0), "0 B/s");
    }
}