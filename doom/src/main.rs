use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long};


#[allow(non_camel_case_types)]
pub type c_long_double = ::std::os::raw::c_double; //?

// C libraries
extern "C" {
    // d_main.c
    fn D_DoomMain() -> !;

    // m_argv.c
    static mut myargc: c_int;
    static mut myargv: *const *const c_char;
}

// Macros to print to JavaScript Console.
use doom::{log, println};




static mut SINGLE_THREAD_ERRNO: c_int = 0; // YOLO
#[no_mangle]
extern "C" fn ___errno_location() -> *const c_int {
    unsafe { &SINGLE_THREAD_ERRNO }
}




// struct timeval { time_t tv_sec; suseconds_t tv_usec; };
#[repr(C)]
struct Timeval {
    tv_sec: c_long, // TODO is this i32 or i64??
    tv_usec: c_long,
}

#[link(wasm_import_module = "js")]
extern "C" {
    fn js_timeofday(ptr: *mut Timeval);
}

#[no_mangle]
extern "C" fn gettimeofday(tv: *mut Timeval, _tz: i32) -> c_int {
    // timezone is obsolete and should not be needef for doom.
    let tv = match unsafe { tv.as_mut() } {
        None => return 0, /*do nothing*/
        Some(tv) => tv,
    };

    unsafe { js_timeofday(tv) };
    log!("gettimeofday slightly unimplemented (TODO: required for Doom's ticks)");
    0 // success
}

fn main() {
    log!(
        "Hello, {}! Answer={} ({:b} in binary)",
        "World, from JS Console",
        42,
        42
    );

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
        log!("panic occurred: \"{}\" {}\n{:?}", p, l, panic_info);
    }));

    println!("Hello, world from rust! (println! working)");

    // TODO: set global variables
    // myargc=2 and myargv={"-2"}

    let binary_name = CString::new("linuxxdoom").unwrap();
    let first_commandline_arg = CString::new("-2").unwrap();
    let argv: [*const c_char; 2] = [binary_name.as_ptr(), first_commandline_arg.as_ptr()];
    unsafe {
        myargc = argv.len() as c_int;
        myargv = &argv as *const *const c_char;
        D_DoomMain();
    };
}
