.PHONY: all test slug py ts rust
all: slug

slug:
	@python py/slugify.py hello world

test:
	cargo test --manifest-path rust/Cargo.toml