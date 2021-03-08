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
macro_rules! print { ($($arg:tt),*) => { log!( $( $arg )* ) }; }


fn main() {
    println!("Hello, world from rust!");
    unsafe { c_main(); }
}
