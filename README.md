# toolkit

Small CLI utilities for everyday shell work. Each tool is a single file with a focused job: encode, slugify, or generate ids. Rust, Python, and Node.js/Bun share the same behavior.

## Tools

| Tool | Language | Entry | What it does |
| --- | --- | --- | --- |
| `toolkit` | Rust | `rust/src/main.rs` | CLI with `slugify`, `b64encode`, `b64decode`, `slugid` subcommands |

## Status

More tools land through pull requests: base64 handling in TypeScript (Bun), slugify in Python, and CI for all three runtimes.

## Development

```sh
cd rust
cargo test
```
