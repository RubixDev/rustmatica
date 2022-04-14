build:
	cargo build --release

run:
	cargo run

test:
	cargo test
	wasm-pack test --node --profile wasm-test
