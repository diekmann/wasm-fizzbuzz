fizzbuzz.wasm: fizzbuzz.wat
	 wat2wasm fizzbuzz.wat

run_wasm: fizzbuzz.wasm
	wasm-interp fizzbuzz.wasm

clean:
	rm -rf fizzbuzz.wasm
