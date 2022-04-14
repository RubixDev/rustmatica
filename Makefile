build:
	cargo build --release

update:
	cargo update
	./run_data_extractor.bash
	python make_blocks.py

test:
	cargo test
	wasm-pack test --node --profile wasm-test
