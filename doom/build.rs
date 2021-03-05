use std::process::Command;

static DOON_SRC: &str = "linuxdoom-1.10";
static LIBC_SRC: &str = "musl-1.2.2";

fn main() {
    println!("cargo:rerun-if-changed={}", DOON_SRC);

    let status = Command::new("make").args(&["-C", DOON_SRC, "linux/liblinuxxdoom.a"]).status().expect("failed to start doom make");
    if !status.success(){
        panic!("Failed to make: {}", status);
    }

    println!("cargo:rustc-link-search={}/linux", DOON_SRC);
    println!("cargo:rustc-link-lib=linuxxdoom");


    println!("cargo:rerun-if-changed={}", LIBC_SRC);
    let status = Command::new("make").args(&["-C", LIBC_SRC, "lib/libc.a"]).status().expect("failed to start musl libc make");
    if !status.success(){
        panic!("Failed to make: {}", status);
    }
    println!("cargo:rustc-link-search={}/lib/", LIBC_SRC);
    println!("cargo:rustc-link-lib=c");

    // libraries which should be removed before going to wasm
    //println!("cargo:rustc-link-lib=X11");
}
