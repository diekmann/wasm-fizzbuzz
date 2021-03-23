use std::os::raw::{c_char, c_int};

#[allow(non_camel_case_types)]
pub type c_wchar = ::std::os::raw::c_long;

// generated

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
