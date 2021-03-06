use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long};

pub type c_wchar_t = ::std::os::raw::c_long;

extern "C" {
    // d_main.c
    fn D_DoomMain() -> !;

    // m_argv.c
    static mut myargc: c_int;
    static mut myargv: *const *const c_char;
}


#[no_mangle]
pub extern "C" fn wctomb(_: *const c_char, _: c_wchar_t) -> c_int {
    panic!("wctomb unimplemented");
}

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
