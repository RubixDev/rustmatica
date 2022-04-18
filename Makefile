build:
	cargo build --release

update:
	cargo update
	./run_data_extractor.bash
	python make_lists.py

test:
	cargo test --profile no-opt --all-features
	cargo test --profile no-opt --no-default-features
	wasm-pack test --node --all-features
	wasm-pack test --node --no-default-features
