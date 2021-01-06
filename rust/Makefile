fizzbuzz.wasm: src/*.rs
	 cargo build --target wasm32-unknown-unknown --release
	 # sudo apt install binaryen
	 wasm-opt -Oz -o fizzbuzz.wasm target/wasm32-unknown-unknown/release/rust.wasm

run_wasm: fizzbuzz.wasm
	python3 -m http.server --bind 127.0.0.1

clean:
	cargo clean
	rm -rf fizzbuzz.wasm
