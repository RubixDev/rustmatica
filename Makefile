build:
	cargo build --release

update:
	cargo update
	./run_data_extractor.bash
	python make_lists.py

test:
	cargo test --all-features
	cargo test --no-default-features
	wasm-pack test --node --all-features
	wasm-pack test --node --no-default-features
