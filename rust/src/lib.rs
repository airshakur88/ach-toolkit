/// Toolkit library: id and slug helpers reused by the CLI.
pub fn slugify(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    out
}

/// Deterministic short hex id from a seed counter (no clock, no OS rng).
pub fn slugid(n: u64) -> String {
    format!("id-{:x}", n.wrapping_mul(0x2545_f491_4f6c_dd1d))
}