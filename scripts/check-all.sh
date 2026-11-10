#!/usr/bin/env bash
# Verify each tool matches its sibling implementations.
set -euo pipefail

expect=$(printf 'ok' | base64)

for impl in "rust --manifest-path rust/Cargo.toml" "ts/base64.ts"; do
  case "$impl" in
    rust*) got=$(cargo run --quiet --manifest-path rust/Cargo.toml -- b64encode ok) ;;
    ts*)   got=$(bun run ts/base64.ts b64encode ok) ;;
  esac
  [[ "$got" == "$expect" ]] || { echo "mismatch in $impl"; exit 1; }
done

echo "all implementations match"