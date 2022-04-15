build:
	cargo build --release

update:
	cargo update
	./run_data_extractor.bash
	python make_lists.py

test:
	cargo test
	wasm-pack test --node
