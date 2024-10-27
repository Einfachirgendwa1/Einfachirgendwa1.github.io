test:
	wasm-pack build --target web
	cd test_locally; cargo run
