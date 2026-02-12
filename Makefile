build:
	cargo build

release:
	cargo build --release

test:
	cargo test

install:
	cargo install --path .

run:
	cargo run -- run --repo . --contract ./fmt_contract.json
