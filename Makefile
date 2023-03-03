build:
	cargo build --release

update:
	cargo update
	./run_data_extractor.sh
	python make_lists.py
	./generate_sources.sh
	cd source-parser && npm start

test:
	cargo test tests:: --all-features -- --nocapture
	cargo test tests:: --profile no-opt --no-default-features -- --nocapture
	git restore test_files/donut.litematic
	git restore test_files/donut_modified.litematic
	wasm-pack test --node --all-features --lib -- tests::
	wasm-pack test --node --profile no-opt --no-default-features --lib -- tests::
