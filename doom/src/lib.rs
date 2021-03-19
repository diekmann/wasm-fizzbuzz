#[macro_use]
extern crate lazy_static;

pub mod js_imports;
mod malloc;
mod printf;
mod unimplemented_libc;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        let __the_log_str = format!( $( $arg )* );
        unsafe { $crate::js_imports::console_log(__the_log_str.as_ptr(), __the_log_str.len()) }
    }
}

#[macro_export]
macro_rules! println { ($($arg:tt),*) => { log!( $( $arg )* ) }; }
