# pull-request-batch

Branches under `features/*` add one cross-language tool each, matching the
`slugify | b64encode | b64decode` interface already on `main`.

| PR | Tool | Language | File |
| --- | --- | --- | --- |
| Add Python slugify | slugify | Python | `py/slugify.py` |
| Add TypeScript base64 | b64 | TypeScript | `ts/base64.ts` |
| Add Rust id generator | slugid | Rust | `rust/src/lib.rs` |
| CI for bun | test | CI | `.github/workflows/ci.yml` |