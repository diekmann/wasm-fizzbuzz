BUILDDIR = target/wasm32-unknown-unknown/release

doom.wasm: src/*.rs clang_compiler_rt/* linuxdoom-1.10/*  musl-1.2.2/*
	 cargo build --release
	 # As log as wasm-ld does not look like it supports LTO for C/Rust cross-language LTO, binaryen is the best we have.
	 # TODO: use https://doc.rust-lang.org/rustc/linker-plugin-lto.html once it works for wasm.
	 wasm-opt -O3 -o doom.wasm ${BUILDDIR}/doom.wasm

run_wasm: doom.wasm
	python3 -m http.server --bind 127.0.0.1

clean:
	cargo clean
	rm -rf doom.wasm
	make -C linuxdoom-1.10 clean
	make -C musl-1.2.2 clean
