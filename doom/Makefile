BUILDDIR = target/wasm32-unknown-unknown/release

fizzbuzz.wasm: src/*.rs
	 cargo build --release
	 # TODO: wasm-opt?
	 cp ${BUILDDIR}/doom.wasm doom.wasm

run_wasm: doom.wasm
	python3 -m http.server --bind 127.0.0.1

clean:
	cargo clean
	rm -rf doom.wasm
	make -C linuxdoom-1.10 clean
	make -C musl-1.2.2 clean
