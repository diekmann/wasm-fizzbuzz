use std::ffi::CString;
use std::ffi::{CStr};
use std::os::raw::{c_char, c_int, c_long};

#[allow(non_camel_case_types)]
pub type c_wchar = ::std::os::raw::c_long;

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

#[no_mangle]
extern "C" fn wctomb(_: *const c_char, _: c_wchar) -> c_int {
    panic!("wctomb unimplemented");
}

#[no_mangle]
extern "C" fn frexpl(_: i32, _: i64, _: i64, _: i32) {
    // type??
    panic!("frexpls unimplemented");
}

#[no_mangle]
extern "C" fn fabsl(_: i32, _: i64, _: i64) {
    // type??
    panic!("fabsl unimplemented");
}

static HOME_ENV: &'static [u8; 11] = b"/home/doom\0"; // C string, terminate with \0! //TODO: use CStr safely here?

#[no_mangle]
extern "C" fn getenv(name: *const c_char) -> *const c_char { //TODO returning an ffi-safe Option<non-nullable> would be cool!!
    // TODO type!!!
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_str().expect("invalid UTF8 getenv call");
    let result = match name {
        "DOOMWADDIR" => std::ptr::null(),
        "HOME" => HOME_ENV.as_ptr() as *const c_char,
        _ => {
            log!("unexepcted getenv({:?}) call", name);
            std::ptr::null()
        }
    };
    result
}

static mut SINGLE_THREAD_ERRNO: c_int = 0; // YOLO
#[no_mangle]
extern "C" fn ___errno_location() -> *const c_int {
    unsafe { &SINGLE_THREAD_ERRNO }
}

#[no_mangle]
extern "C" fn access(pathname: *const c_char, mode: c_int) -> c_int {
    const ENOENT: c_int = 2;

    let pathname = unsafe { CStr::from_ptr(pathname).to_str().expect("invalid UTF8") };
    match pathname {
        "./doom2f.wad" => ENOENT,
        "./doom2.wad" => ENOENT,
        "./plutonia.wad" => ENOENT,
        "./tnt.wad" => ENOENT,
        "./doom.wad" => ENOENT,
        "./doomu.wad" => ENOENT,
        "./doom1.wad" => 0, /* OK */
        _ => panic!("access({}, {}) unimplemented", pathname, mode),
    }
}

#[no_mangle]
extern "C" fn fopen(pathname: *const c_char, mode: c_int) -> i32 /* FILE* */ {
    let pathname = unsafe { CStr::from_ptr(pathname).to_str().expect("invalid UTF8") };

    if pathname == "/home/doom/.doomrc" {
        return 0; // NULL for error
    }

    panic!("fopen({}, {}) unimplemented", pathname, mode);
}

const DOOM1_WAD_FD: c_int = 42; // file descriptor for openend ./doom1.wad
static DOOM1_WAD: &[u8; 4196020] = include_bytes!("../doom1.wad");
static mut DOOM1_WAD_SEEKER: usize = 0;

#[no_mangle]
extern "C" fn open(pathname: *const c_char, flags: c_int, mode: i32) -> i32 {
    let pathname = unsafe { CStr::from_ptr(pathname).to_str().expect("invalid UTF8") };

    if pathname == "./doom1.wad" {
        return DOOM1_WAD_FD;
    }

    panic!("open({}, {}, {}) unimplemented", pathname, flags, mode);
}

#[no_mangle]
extern "C" fn read(fd: c_int, buf: *mut u8 /*TODO is c_char*/, count: usize) -> isize {
    if fd == DOOM1_WAD_FD {
        //TODO read DOOM1_WAD and advance seek
        let buf = unsafe { std::slice::from_raw_parts_mut(buf, count) };
        let s = unsafe { DOOM1_WAD_SEEKER };
        buf[..count].copy_from_slice(&DOOM1_WAD[s..s + count]);
        unsafe {
            DOOM1_WAD_SEEKER += count;
        }
        return count as isize;
    }
    panic!("read({}, buf, {}) unimplemented", fd, count);
}

#[no_mangle]
extern "C" fn lseek(fd: i32, offset: i64, whence: c_int) -> i64 {
    const SEEK_SET: c_int = 0;
    const SEEK_CUR: c_int = 1;
    const SEEK_END: c_int = 2;
    if fd == DOOM1_WAD_FD {
        match whence {
            SEEK_SET => {
                unsafe { DOOM1_WAD_SEEKER = offset as usize };
                return unsafe { DOOM1_WAD_SEEKER } as i64;
            }
            _ => {
                log!("TODO lseek");
            }
        }
    }
    panic!("lseek({}, {}, {}) unimplemented", fd, offset, whence);
}

#[no_mangle]
extern "C" fn I_ShutdownGraphics() {
    log!("Bye!! TODO: implement I_ShutdownGraphics");
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
        None => return 0 /*do nothing*/ ,
        Some(tv) => tv,
    };

    unsafe { js_timeofday(tv) };
    log!("gettimeofday slightly unimplemented (TODO: required for Doom's ticks)");
    0 // success
}


#[no_mangle]
extern "C" fn I_InitGraphics() {
    log!("I_InitGraphics (TODO)");
}

#[no_mangle]
extern "C" fn I_StartFrame() {
    // From the original Doom Sources:
    // https://github.com/id-Software/DOOM/blob/master/linuxdoom-1.10/i_video.c#L185
    // // er?
}

#[no_mangle]
extern "C" fn I_StartTic() {
    log!("I_StartTic unimplemented!!!!!!!!!!!!!!!!!!");
    // should get inputs (e.g. key presses/releases)
    // and send them to D_PostEvent().
}

#[no_mangle]
extern "C" fn I_UpdateNoBlit() {
    // From the original Doom sources:
    // https://github.com/id-Software/DOOM/blob/master/linuxdoom-1.10/i_video.c#L346
    // // what is this?
}

#[no_mangle]
extern "C" fn I_SetPalette(_: i32) {
    log!("I_SetPalette unimplemented");
}

// C libraries
extern "C" {
    // v_video.h
    static screens: [*const u8; 5];
}

#[link(wasm_import_module = "js")]
extern "C" {
    // ptr points to a SCREENWIDTH*SCREENHEIGHT array.
    // Each byte represents one 256-color PseudoColor Pixel.
    fn js_draw_screen(ptr: *const u8);
}

#[no_mangle]
extern "C" fn I_FinishUpdate() {
    // Draws the screen
    // Doom's C sources define
    //
    // // Screen 0 is the screen updated by I_Update screen.
    // // Screen 1 is an extra buffer.
    // extern	byte*		screens[5];
    //
    // I think only screens[0] is needed.
    // The screens are SCREENWIDTH*SCREENHEIGHT, which is 320x200
    unsafe {
        let the_screen = screens[0];
        js_draw_screen(the_screen);
    }
    panic!("I_FinishUpdate unimplemented");
}

#[no_mangle]
extern "C" fn I_ReadScreen(_: i32) {
    panic!("I_ReadScreen unimplemented");
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
