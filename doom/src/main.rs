use std::os::raw::{c_int};
use std::ptr;

#[macro_use]
extern crate lazy_static;

#[allow(non_camel_case_types)]
pub type c_long_double = ::std::os::raw::c_double; //?

// C libraries
extern "C" {
    // d_main.c
    fn D_DoomMain();
    fn D_DoomLoop_loop();

    // m_argv.c
    static mut myargc: c_int;
    static mut myargv: *const *const u8; //c_char;
}

// Macros to print to JavaScript Console.
use doom::{log, println};

static mut SINGLE_THREAD_ERRNO: c_int = 0; // YOLO
#[no_mangle]
extern "C" fn ___errno_location() -> *const c_int {
    unsafe { &SINGLE_THREAD_ERRNO }
}


#[link(wasm_import_module = "js")]
extern "C" {
    // i32 timestamps in milliseconds should be enough for over 500hours of DooM.
    fn js_milliseconds_since_start() -> i32;
}

//
// I_GetTime
// Doom comment says: returns time in 1/70th second tics.
// But it's actually more 1/35 second tics, i.e. optimized for 35FPS
// I.e. one tic every 28.572ms.
// Returns a monotonically increasing number of ticks.
//
#[no_mangle]
extern "C" fn I_GetTime() -> c_int {
    const TICRATE: c_int = 35;

    let ms = unsafe { js_milliseconds_since_start() };

    // Basically, there should be no need to record a basetime,
    // since performance.now() shold roughly start at 0.
    static mut BASETIME: c_int = 0;
    let base = unsafe {
        if BASETIME == 0 {
            BASETIME = ms;
            crate::log!("BASETIME initialized to {}", BASETIME);
        }
        BASETIME
    };
    (ms-base)*TICRATE/1000
}


lazy_static! {
    // ARGV must have 'static lifetime, since Doom may look at it at any point.
    // The type signature ensures that the argv we are constructing lives forever.
    // leaks memory, so it should only be called once.
    static ref SAFE_ARGV: &'static [&'static [u8]] = {
        // C strings end with zero
        let argv0 = b"linuxxdoom\0";
        let argv = vec![&argv0[..]];
        argv.leak()
    };
}

// only call once, leaks memory, because the argv we point to must live forever.
fn make_c_argv() -> *const *const u8 {
    let mut argv: std::vec::Vec<*const u8> = SAFE_ARGV.iter().map(|s| s.as_ptr()).collect();
    argv.push(ptr::null()); // Calling convention compatibility: a final NULL separates argv from envp.
    argv.leak().as_ptr()
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

    println!("Hello, world from rust! 🦀🦀🦀 (println! working)");

    // TODO: better set global variables and keep them alive forever.
    unsafe {
        myargc = SAFE_ARGV.len() as c_int;
        myargv = make_c_argv();
        D_DoomMain();
    };
}

#[no_mangle]
pub extern "C" fn doom_loop_step() {
    unsafe { D_DoomLoop_loop() };
}
