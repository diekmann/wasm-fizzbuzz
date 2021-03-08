// $ make libmain.a && cargo clean && cargo run

use std::ffi::CStr;
use std::os::raw::{c_char};

extern "C" {
    fn c_main();
}

// JavaScript
extern "C" {
    fn console_log(ptr: *const u8, len: usize);
}


macro_rules! log {
    ($($arg:tt)*) => {
        let __the_log_str = format!( $( $arg )* );
        unsafe { console_log(__the_log_str.as_ptr(), __the_log_str.len()) }
    }
}

macro_rules! println { ($($arg:tt),*) => { log!( $( $arg )* ) }; }

#[no_mangle]
extern "C" fn hello_from_rust(name: *const c_char) {
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_str().expect("invalid UTF8 hello_from_rust call");
    log!("Hello, \"{}\", nice to meet you!", name);
}

fn main() {
    std::panic::set_hook(Box::new(|panic_info| {
        log!("PANIC!!");
        let p = match panic_info.payload().downcast_ref::<&str>() {
            Some(s) => s.to_string(),
            None => String::from("<no further information>"),
        };
        let l = match panic_info.location() {
            Some(l) => format!("in file '{}' at line {}", l.file(), l.line()),
            None => String::from("but can't get location information..."),
        };
        log!("panic occurred: \"{}\" {}", p, l);
    }));

    println!("Hello, world from rust!");
    unsafe { c_main(); }

}

struct IOVec {
    iov_base: *const u8, // void*
    iov_len: usize, // size_t
}


#[no_mangle]
extern "C" fn  __syscall3(n: i32, a1: i32, a2: i32, a3: i32) -> i32{
    if n==20 /*SYS_writev*/ && a1 == 1 /*STDOUT*/ {
        log!("SYS_writev to STDOUT");

        let iov_ptr: *const IOVec = a2 as *const IOVec;
        let iovcnt = a3 as usize;
        let iovs = unsafe { std::slice::from_raw_parts(iov_ptr, iovcnt) };
        let mut bytes_written = 0;
        for iov in iovs {
            unsafe { console_log(iov.iov_base, iov.iov_len) };
            bytes_written += iov.iov_len as i32;
        }
        return bytes_written;
    }else{
        log!("other __syscall3");
    }
    return -1;
}
