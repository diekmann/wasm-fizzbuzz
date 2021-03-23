use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

const DOOM1_WAD_FD: c_int = 42; // file descriptor for openend ./doom1.wad
static DOOM1_WAD: &[u8; 4196020] = include_bytes!("../doom1.wad");
static mut DOOM1_WAD_SEEKER: usize = 0;

static HOME_ENV: &'static [u8; 11] = b"/home/doom\0"; // C string, terminate with \0! //TODO: use CStr safely here?

#[no_mangle]
extern "C" fn getenv(name: *const c_char) -> *const c_char {
    //TODO returning an ffi-safe Option<non-nullable> would be cool!!
    // TODO type!!!
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_str().expect("invalid UTF8 getenv call");
    let result = match name {
        "DOOMWADDIR" => std::ptr::null(),
        "HOME" => HOME_ENV.as_ptr() as *const c_char,
        _ => {
            crate::log!("unexepcted getenv({:?}) call", name);
            std::ptr::null()
        }
    };
    result
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
    const _SEEK_CUR: c_int = 1;
    const _SEEK_END: c_int = 2;
    if fd == DOOM1_WAD_FD {
        match whence {
            SEEK_SET => {
                unsafe { DOOM1_WAD_SEEKER = offset as usize };
                return unsafe { DOOM1_WAD_SEEKER } as i64;
            }
            _ => {
                crate::log!("TODO lseek");
            }
        }
    }
    panic!("lseek({}, {}, {}) unimplemented", fd, offset, whence);
}
