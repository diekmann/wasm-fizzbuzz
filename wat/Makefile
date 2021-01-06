fizzbuzz.wasm: fizzbuzz.wat
	 wat2wasm fizzbuzz.wat

run_wasm: fizzbuzz.wasm
	# The good: JavaScript is quite restricted for file:// URLs nowadays.
	# The bad: a local webserver is required to test some frontend coding.
	python3 -m http.server --bind 127.0.0.1

clean:
	rm -rf fizzbuzz.wasm
