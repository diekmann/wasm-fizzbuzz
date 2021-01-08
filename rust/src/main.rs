#![cfg_attr(not(test), no_std)]  // No implicit linking to the std lib, except for tests.
#![cfg_attr(not(test), no_main)] // Outside tests, there is no runtime, we define our own entry point
use core::panic::PanicInfo;

#[link(wasm_import_module = "host")]
extern "C" {
    fn putchar(c: i32);
}

fn puts(str: &[u8]) {
    for c in str.iter() {
        unsafe {putchar(*c as i32)};
    }
    unsafe {putchar('\n' as i32)};
}

// writes ASCII number to buf, returns length of string.
// buf must be large enough. 11 bytes should be enough
// to hold the string representation (including sign) of an i32.
fn int_to_ascii(buf: &mut [u8], mut i: i32) -> usize{
    let mut len: usize = 0;
    let mut div: i32 = 1000000000; // yolo
    if i < 0 {
        buf[len] = b'-';
        len += 1;
        i *= -1; // i should not be INT_MIN.
    }
    while div >= 1 {
        buf[len] = b'0' + ((i / div) as u8);
        i %= div;
        div /= 10;
        len += 1;
    }
    return len
}

#[cfg(test)]
mod tests {
    use std::str;
    use crate::int_to_ascii;

    #[test]
    fn it_works() {
        let mut buf: [u8; 11] = [0; 11];
        let len = int_to_ascii(&mut buf, 42);
        assert_eq!(str::from_utf8(&buf[0..len]), Ok("0000000042"));

        //TODO: I really like Golang's table-driven tests, can I get the same in rust without macro magic?
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    puts(b"PANIC!");
    loop{};
}

fn fizzbuzz(n: i32){
    let mut buf: [u8; 11] = [0; 11];
    if n % 15 == 0 {
        puts(b"fizzbuzz");
    } else if n % 5 == 0 {
        puts(b"buzz");
    } else if n % 3 == 0 {
        puts(b"fizz");
    } else {
        let length = int_to_ascii(&mut buf, n);
        puts(&buf[0..length]);
    }
}

#[cfg(not(test))]
#[no_mangle]
pub fn main(){
    for n in 0..100 {
        fizzbuzz(n);
    }
}
