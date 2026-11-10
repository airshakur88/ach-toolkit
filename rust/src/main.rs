use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(cmd) = args.next() else {
        eprintln!("usage: toolkit <slugify|b64encode|b64decode|slugid> [input]");
        return ExitCode::FAILURE;
    };
    match cmd.as_str() {
        "slugify" => {
            let input = args.collect::<Vec<_>>().join(" ");
            println!("{}", slugify(&input));
        }
        "b64encode" => {
            let input = args.collect::<Vec<_>>().join(" ");
            use base64::Engine;
            println!("{}", base64::engine::general_purpose::STANDARD.encode(input));
        }
        "b64decode" => {
            let input = args.collect::<Vec<_>>().join(" ");
            use base64::Engine;
            match base64::engine::general_purpose::STANDARD.decode(input) {
                Ok(bytes) => println!("{}", String::from_utf8_lossy(&bytes)),
                Err(e) => {
                    eprintln!("invalid base64: {e}");
                    return ExitCode::FAILURE;
                }
            }
        }
        "slugid" => {
            println!("{}", slugify(&format!("id-{}", rand_hex(8))));
        }
        _ => {
            eprintln!("unknown command: {cmd}");
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}

fn slugify(s: &str) -> String {
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

fn rand_hex(n: usize) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    let mut x = seed as u64 ^ 0x9e3779b97f4a7c15;
    let mut s = String::new();
    for _ in 0..n {
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        s.push_str(&format!("{:02x}", (x.wrapping_mul(0x2545f4914f6cdd1d) & 0xff) as u8));
    }
    s
}