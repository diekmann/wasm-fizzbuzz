use std::os::raw::c_int;

#[repr(C)]
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
        let out = if a1 == STDOUT {
            crate::js_imports::js_stdout
        } else {
            crate::js_imports::js_stderr
        };
        let iov_ptr: *const IOVec = a2 as *const IOVec;
        let iovcnt = a3 as usize;
        let iovs = unsafe { std::slice::from_raw_parts(iov_ptr, iovcnt) };
        let mut bytes_written = 0;
        for iov in iovs {
            if iov.iov_len == 0 {
                continue;
            }
            unsafe { out(iov.iov_base, iov.iov_len) };
            bytes_written += iov.iov_len as i32;
        }
        return bytes_written;
    } else {
        crate::log!("other __syscall3({}, {}, {}, {})", n, a1, a2, a3);
    }
    return -1;
}
