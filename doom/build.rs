use std::process::Command;

static DOON_SRC: &str = "linuxdoom-1.10";

fn main() {
    println!("cargo:rerun-if-changed={}", DOON_SRC);

    let status = Command::new("make").args(&["-C", DOON_SRC, "linux/liblinuxxdoom.a"]).status().expect("failed to start make");
    if !status.success(){
        panic!("Failed to make: {}", status);
    }

    println!("cargo:rustc-link-search={}/linux", DOON_SRC);
    println!("cargo:rustc-link-lib=linuxxdoom");


    // libraries which should be removed before going to wasm
    println!("cargo:rustc-link-lib=X11");
}
