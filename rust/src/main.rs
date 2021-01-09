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
    use super::*;

    macro_rules! assert_int_to_ascii {
        ($input:expr, $want:expr) => {
            let mut buf: [u8; 11] = [0; 11];
            let length = int_to_ascii(&mut buf, $input);
            let got = str::from_utf8(&buf[..length]);
            assert_eq!(got, Ok($want));
        }
    }

    #[test]
    fn int_to_ascii_positive() {
        assert_int_to_ascii!(0, "0000000000");
        assert_int_to_ascii!(42, "0000000042");
        assert_int_to_ascii!(15, "0000000015");
        assert_int_to_ascii!(2147483647, "2147483647");
        assert_int_to_ascii!(std::i32::MAX, "2147483647");
    }

    #[test]
    fn int_to_ascii_negative() {
        assert_int_to_ascii!(-0, "0000000000");
        assert_int_to_ascii!(-42, "-0000000042");
        assert_int_to_ascii!(-15, "-0000000015");
        assert_int_to_ascii!(-2147483647, "-2147483647");
        assert_int_to_ascii!(-std::i32::MAX, "-2147483647");
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn int_to_ascii_bug() {
        assert_int_to_ascii!(std::i32::MIN, "-2147483648");
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    puts(b"PANIC!");
    loop{};
}

const FIZZBUZZ: &[u8; 8] = b"fizzbuzz";

fn fizzbuzz(n: i32){
    let mut buf: [u8; 11] = [0; 11];
    if n % 15 == 0 {
        puts(FIZZBUZZ);
    } else if n % 5 == 0 {
        puts(&FIZZBUZZ[4..]); // buzz
    } else if n % 3 == 0 {
        puts(&FIZZBUZZ[..4]); // fizz
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
