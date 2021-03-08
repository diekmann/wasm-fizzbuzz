use std::ffi::CString;
use std::ffi::{c_void, CStr};
use std::os::raw::{c_char, c_int};

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

// JavaScript
#[link(wasm_import_module = "js")]
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

static HOME_ENV: &'static [u8; 11] = b"/home/doom\0"; // C string, terminate with \0!

#[no_mangle]
extern "C" fn getenv(name: *const c_char) -> Option<&'static [u8; 11]> {
    // TODO type!!!
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_str().expect("invalid UTF8 getenv call");
    let result = match name {
        "DOOMWADDIR" => None,
        "HOME" => Some(HOME_ENV),
        _ => {
            log!("unexepcted getenv({:?}) call", name);
            None
        }
    };
    result
}

struct IOVec {
    iov_base: *const u8, // void*
    iov_len: usize,      // size_t
}

#[no_mangle]
extern "C" fn __syscall3(n: i32, a1: i32, a2: i32, a3: i32) -> i32 {
    const SYS_WRITEV: c_int = 20;

    const STDOUT: c_int = 1;
    const STDERR: c_int = 2;

    if n == SYS_WRITEV && (a1 == STDOUT || a1 == STDERR) {
        let iov_ptr: *const IOVec = a2 as *const IOVec;
        let iovcnt = a3 as usize;
        let iovs = unsafe { std::slice::from_raw_parts(iov_ptr, iovcnt) };
        let mut bytes_written = 0;
        for iov in iovs {
            if iov.iov_len == 0 {
                continue;
            }
            unsafe { console_log(iov.iov_base, iov.iov_len) };
            bytes_written += iov.iov_len as i32;
        }
        return bytes_written;
    } else {
        log!("other __syscall3({}, {}, {}, {})", n, a1, a2, a3);
    }
    return -1;
}

#[no_mangle]
extern "C" fn malloc(size: usize) -> *const c_void {
    let mut mem: Vec<u8> = std::vec::Vec::with_capacity(size);
    unsafe { mem.set_len(size) };
    let static_ref: &'static mut [u8] = mem.leak(); //TODO make free()-able.
    static_ref as *mut [u8] as *mut c_void
}

#[no_mangle]
extern "C" fn free(_: i32) {
    panic!("free unimplemented");
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

// generated

#[no_mangle]
extern "C" fn I_ReadScreen(_: i32) {
    panic!("I_ReadScreen unimplemented");
}

#[no_mangle]
extern "C" fn __lockfile(_: i32) -> i32 {
    panic!("__lockfile unimplemented");
}

#[no_mangle]
extern "C" fn __unlockfile(_: i32) {
    panic!("__unlockfile unimplemented");
}

#[no_mangle]
extern "C" fn __signbitl(_: i64, _: i64) -> i32 {
    panic!("__signbitl unimplemented");
}

#[no_mangle]
extern "C" fn __fpclassifyl(_: i64, _: i64) -> i32 {
    panic!("__fpclassifyl unimplemented");
}

#[no_mangle]
extern "C" fn strerror(_: i32) -> i32 {
    panic!("strerror unimplemented");
}

#[no_mangle]
extern "C" fn I_UpdateNoBlit() {
    panic!("I_UpdateNoBlit unimplemented");
}

#[no_mangle]
extern "C" fn I_SetPalette(_: i32) {
    panic!("I_SetPalette unimplemented");
}

#[no_mangle]
extern "C" fn I_FinishUpdate() {
    panic!("I_FinishUpdate unimplemented");
}

#[no_mangle]
extern "C" fn gettimeofday(_: i32, _: i32) -> i32 {
    panic!("gettimeofday unimplemented");
}

#[no_mangle]
extern "C" fn exit(_: i32) {
    panic!("exit unimplemented");
}

#[no_mangle]
extern "C" fn usleep(_: i32) -> i32 {
    panic!("usleep unimplemented");
}

#[no_mangle]
extern "C" fn __stdio_close() {
    panic!("__stdio_close unimplemented");
}

#[no_mangle]
extern "C" fn __stdio_seek() {
    panic!("__stdio_seek unimplemented");
}

#[no_mangle]
extern "C" fn __lock(_: i32) {
    panic!("__lock unimplemented");
}

#[no_mangle]
extern "C" fn __unlock(_: i32) {
    panic!("__unlock unimplemented");
}

#[no_mangle]
extern "C" fn I_InitGraphics() {
    panic!("I_InitGraphics unimplemented");
}

#[no_mangle]
extern "C" fn I_StartFrame() {
    panic!("I_StartFrame unimplemented");
}

#[no_mangle]
extern "C" fn I_StartTic() {
    panic!("I_StartTic unimplemented");
}

#[no_mangle]
extern "C" fn __toread(_: i32) -> i32 {
    panic!("__toread unimplemented");
}

#[no_mangle]
extern "C" fn close(_: i32) -> i32 {
    panic!("close unimplemented");
}

#[no_mangle]
extern "C" fn write(_: i32, _: i32, _: i32) -> i32 {
    panic!("write unimplemented");
}

#[no_mangle]
extern "C" fn fstat(_: i32, _: i32) -> i32 {
    panic!("fstat unimplemented");
}

#[no_mangle]
extern "C" fn __uflow(_: i32) -> i32 {
    panic!("__uflow unimplemented");
}

#[no_mangle]
extern "C" fn mbrtowc(_: i32, _: i32, _: i32, _: i32) -> i32 {
    panic!("mbrtowc unimplemented");
}

#[no_mangle]
extern "C" fn realloc(_: i32, _: i32) -> i32 {
    panic!("realloc unimplemented");
}

#[no_mangle]
extern "C" fn mbsinit(_: i32) -> i32 {
    panic!("mbsinit unimplemented");
}

#[no_mangle]
extern "C" fn scalbn(_: f64, _: i32) -> f64 {
    panic!("scalbn unimplemented");
}

#[no_mangle]
extern "C" fn copysignl(_: i32, _: i64, _: i64, _: i64, _: i64) {
    panic!("copysignl unimplemented");
}

#[no_mangle]
extern "C" fn scalbnl(_: i32, _: i64, _: i64, _: i32) {
    panic!("scalbnl unimplemented");
}

#[no_mangle]
extern "C" fn fmodl(_: i32, _: i64, _: i64, _: i64, _: i64) {
    panic!("fmodl unimplemented");
}

// end generated

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

    println!("Hello, world from rust!");

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
