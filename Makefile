build:
	cargo build --release

update:
	cargo update
	./run_data_extractor.bash
	python make_lists.py

test:
	cargo test tests:: --profile no-opt --all-features -- --nocapture
	cargo test tests:: --profile no-opt --no-default-features -- --nocapture
	git restore test_files/donut.litematic
	git restore test_files/donut_modified.litematic
	wasm-pack test --node --all-features --lib -- tests::
	wasm-pack test --node --no-default-features --lib -- tests::
