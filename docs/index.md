# Toolkit

## Tools

Each command reads input as a single string and prints the result to stdout.

| Command | Languages | Example |
| --- | --- | --- |
| `slugify` | rust, ts, py | `toolkit slugify "Hello World"` -> `hello-world` |
| `b64encode` | rust, ts, py | `toolkit b64encode "ok"` -> `b2s=` |
| `b64decode` | rust, ts, py | `toolkit b64decode "b2s="` -> `ok` |

## Runtimes

- Rust: `cargo run --manifest-path rust/Cargo.toml -- <cmd> <input>`
- Bun: `bun run ts/base64.ts <b64encode|b64decode> <input>`
- Python: `python py/cli.py <cmd> <input>`