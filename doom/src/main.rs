use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long, c_double};

#[allow(non_camel_case_types)]
pub type c_wchar = ::std::os::raw::c_long;

#[allow(non_camel_case_types)]
pub type c_long_double = ::std::os::raw::c_double; //?

extern "C" {
    // d_main.c
    fn D_DoomMain() -> !;

    // m_argv.c
    static mut myargc: c_int;
    static mut myargv: *const *const c_char;
}


#[no_mangle]
extern "C" fn wctomb(_: *const c_char, _: c_wchar) -> c_int {
    panic!("wctomb unimplemented");
}

#[no_mangle]
extern "C" fn frexpl(_: i32, _: i64, _: i64, _: i32) { // type??
    panic!("frexpls unimplemented");
}

#[no_mangle]
extern "C" fn fabsl(_: i32, _: i64, _: i64) { // type??
    panic!("fabsl unimplemented");
}


// generated

#[no_mangle]
extern "C" fn I_ReadScreen(_: i32) {
    panic!("I_ReadScreen unimplemented");
}

#[no_mangle]
extern "C" fn ___errno_location() -> i32 {
    panic!("___errno_location unimplemented");
}

#[no_mangle]
extern "C" fn __lockfile(_: i32) -> i32 {
    panic!("__lockfile unimplemented");
}

#[no_mangle]
extern "C" fn __towrite(_: i32) -> i32 {
    panic!("__towrite unimplemented");
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
extern "C" fn malloc(_: i32) -> i32 {
    panic!("malloc unimplemented");
}

#[no_mangle]
extern "C" fn gettimeofday(_: i32, _: i32) -> i32 {
    panic!("gettimeofday unimplemented");
}

#[no_mangle]
extern "C" fn I_ShutdownGraphics() {
    panic!("I_ShutdownGraphics unimplemented");
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
extern "C" fn __stdio_write() {
    panic!("__stdio_write unimplemented");
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
extern "C" fn __stdout_write() {
    panic!("__stdout_write unimplemented");
}

#[no_mangle]
extern "C" fn fopen(_: i32, _: i32) -> i32 {
    panic!("fopen unimplemented");
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
extern "C" fn getenv(_: i32) -> i32 {
    panic!("getenv unimplemented");
}

#[no_mangle]
extern "C" fn access(_: i32, _: i32) -> i32 {
    panic!("access unimplemented");
}

#[no_mangle]
extern "C" fn __toread(_: i32) -> i32 {
    panic!("__toread unimplemented");
}

#[no_mangle]
extern "C" fn free(_: i32) {
    panic!("free unimplemented");
}

#[no_mangle]
extern "C" fn fputs(_: i32, _: i32) -> i32 {
    panic!("fputs unimplemented");
}

#[no_mangle]
extern "C" fn __overflow(_: i32, _: i32) -> i32 {
    panic!("__overflow unimplemented");
}

#[no_mangle]
extern "C" fn open(_: i32, _: i32, _: i32) -> i32 {
    panic!("open unimplemented");
}

#[no_mangle]
extern "C" fn read(_: i32, _: i32, _: i32) -> i32 {
    panic!("read unimplemented");
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

#[no_mangle]
extern "C" fn lseek(_: i32, _: i64, _: i32) -> i64 {
    panic!("lseek unimplemented");
}

// end generated



fn main() {
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
