BUILDDIR = target/wasm32-unknown-unknown/release

fizzbuzz.wasm: src/*.rs
	 cargo build --target wasm32-unknown-unknown --release
	 wasm2wat ${BUILDDIR}/rust.wasm | grep -v '(export "\(memory\|__\)' > ${BUILDDIR}/no_exports.wat
	 wat2wasm -o ${BUILDDIR}/fizzbuzz.wasm ${BUILDDIR}/no_exports.wat
	 # sudo apt install binaryen
	 wasm-opt -Oz -o fizzbuzz.wasm ${BUILDDIR}/fizzbuzz.wasm

run_wasm: fizzbuzz.wasm
	python3 -m http.server --bind 127.0.0.1

clean:
	cargo clean
	rm -rf fizzbuzz.wasm
