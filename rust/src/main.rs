#![cfg_attr(not(test), no_std)] // No implicit linking to the std lib, except for tests.
#![cfg_attr(not(test), no_main)] // Outside tests, there is no runtime, we define our own entry point
use core::panic::PanicInfo;

#[cfg(not(test))]
#[link(wasm_import_module = "host")]
extern "C" {
    fn putchar(c: i32);
}
#[cfg(test)]
use tests::putchar; // use a fake in tests.

fn puts(str: &[u8]) {
    for c in str.iter() {
        unsafe { putchar(*c as i32) };
    }
    unsafe { putchar('\n' as i32) };
}

// writes ASCII number to buf, returns length of string.
// buf must be large enough. 11 bytes should be enough
// to hold the string representation (including sign) of an i32.
fn int_to_ascii(buf: &mut [u8], mut i: i32) -> usize {
    let mut len: usize = 0;
    let mut div: i32 = 1000000000; // yolo
    if i == 0 {
        // special case, since loop below skips leading zeros.
        buf[0] = b'0';
        return 1;
    }
    if i < 0 {
        buf[len] = b'-';
        len += 1;
        i *= -1; // i should not be INT_MIN.
    }
    let mut skip_zeros = true; // do no print leading zeros.
    while div >= 1 {
        let the_digit = i / div;
        i %= div;
        div /= 10;
        skip_zeros = skip_zeros && the_digit == 0;
        if skip_zeros {
            continue;
        }
        buf[len] = b'0' + (the_digit as u8);
        len += 1;
    }
    return len;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    macro_rules! assert_int_to_ascii {
        ($input:expr, $want:expr) => {
            let mut buf: [u8; 11] = [0; 11];
            let length = int_to_ascii(&mut buf, $input);
            let got = str::from_utf8(&buf[..length]);
            assert_eq!(got, Ok($want));
        };
    }

    #[test]
    fn int_to_ascii_positive() {
        assert_int_to_ascii!(0, "0");
        assert_int_to_ascii!(42, "42");
        assert_int_to_ascii!(4200, "4200");
        assert_int_to_ascii!(15, "15");
        assert_int_to_ascii!(2147483647, "2147483647");
        assert_int_to_ascii!(std::i32::MAX, "2147483647");
    }

    #[test]
    fn int_to_ascii_negative() {
        assert_int_to_ascii!(-0, "0");
        assert_int_to_ascii!(-42, "-42");
        assert_int_to_ascii!(-4200, "-4200");
        assert_int_to_ascii!(-15, "-15");
        assert_int_to_ascii!(-2147483647, "-2147483647");
        assert_int_to_ascii!(-std::i32::MAX, "-2147483647");
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn int_to_ascii_bug() {
        assert_int_to_ascii!(std::i32::MIN, "-2147483648");
    }

    // fake putchar implementation for testing.
    static mut PUTCHAR_BUF: [u8; 1024] = [0; 1024];
    static mut PUTCHAR_BUF_IDX: usize = 0;
    pub unsafe fn putchar(c: i32) {
        PUTCHAR_BUF[PUTCHAR_BUF_IDX] = c as u8;
        PUTCHAR_BUF_IDX += 1;
    }

    macro_rules! assert_fizzbuzz {
        ($input:expr, $want:expr) => {
            unsafe {
                // reset fake buffer.
                PUTCHAR_BUF = [0; 1024];
                PUTCHAR_BUF_IDX = 0;
            }
            fizzbuzz($input); // function under test, writing to fake buffer.
            let printed_str = unsafe {
                str::from_utf8(&PUTCHAR_BUF[..PUTCHAR_BUF_IDX]).expect("PUTCHAR_BUF invalid utf8")
            };
            assert_eq!(printed_str, $want);
        };
    }
    #[test]
    fn fizzbuzz_printing() {
        assert_fizzbuzz!(1, "1\n");
        assert_fizzbuzz!(2, "2\n");
        assert_fizzbuzz!(3, "fizz\n");
        assert_fizzbuzz!(4, "4\n");
        assert_fizzbuzz!(5, "buzz\n");
        assert_fizzbuzz!(6, "fizz\n");
        assert_fizzbuzz!(7, "7\n");
        assert_fizzbuzz!(8, "8\n");
        assert_fizzbuzz!(9, "fizz\n");
        assert_fizzbuzz!(10, "buzz\n");
        assert_fizzbuzz!(11, "11\n");
        assert_fizzbuzz!(12, "fizz\n");
        assert_fizzbuzz!(13, "13\n");
        assert_fizzbuzz!(14, "14\n");
        assert_fizzbuzz!(15, "fizzbuzz\n");
        assert_fizzbuzz!(16, "16\n");
        assert_fizzbuzz!(17, "17\n");
        assert_fizzbuzz!(18, "fizz\n");
        assert_fizzbuzz!(19, "19\n");
        assert_fizzbuzz!(20, "buzz\n");
        assert_fizzbuzz!(21, "fizz\n");
        assert_fizzbuzz!(22, "22\n");
        assert_fizzbuzz!(23, "23\n");
        assert_fizzbuzz!(24, "fizz\n");
        assert_fizzbuzz!(25, "buzz\n");
        assert_fizzbuzz!(26, "26\n");
        assert_fizzbuzz!(27, "fizz\n");
        assert_fizzbuzz!(28, "28\n");
        assert_fizzbuzz!(29, "29\n");
        assert_fizzbuzz!(30, "fizzbuzz\n");
        assert_fizzbuzz!(31, "31\n");
        assert_fizzbuzz!(32, "32\n");
        assert_fizzbuzz!(33, "fizz\n");
        assert_fizzbuzz!(34, "34\n");
        assert_fizzbuzz!(35, "buzz\n");
        assert_fizzbuzz!(36, "fizz\n");
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    puts(b"PANIC!");
    loop {}
}

const FIZZBUZZ: &[u8; 8] = b"fizzbuzz";

fn fizzbuzz(n: i32) {
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
pub fn main() {
    for n in 0..100 {
        fizzbuzz(n);
    }
}
