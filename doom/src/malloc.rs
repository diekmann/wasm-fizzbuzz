use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::Mutex;

// wasm32 is single-threaded (currently), so a global variable should be safe.
lazy_static! {
    // They key if i32 is in fact the `*const c_void` retrned by `malloc`.
    static ref MALLOC_IDX: Mutex<HashMap<i32, Vec<u8>>> = Mutex::from(HashMap::new());
}

#[no_mangle]
extern "C" fn malloc(size: usize) -> *const c_void {
    let mut mem: Vec<u8> = std::vec::Vec::with_capacity(size);

    // unsafe { mem.set_len(size) }; // malloc returns uninitialized memory, so this should be fine
    mem.resize(size, 0);

    let p = mem.as_ptr();
    MALLOC_IDX.lock().unwrap().insert(p as i32, mem); // transfer ownership into MALLOC_IDX
    crate::log!("mallocing {} bytes at {}", size, p as i32);
    p as *const c_void
}

#[no_mangle]
extern "C" fn free(ptr: i32) {
    let mut idx = MALLOC_IDX.lock().unwrap();
    match idx.remove(&ptr) {
        None => panic!("Memory {} not malloced", ptr),
        Some(v) => {
            crate::log!("freeing memory at {} (size {})", ptr, v.len());
            // TODO drop v?
        }
    }
}

#[no_mangle]
extern "C" fn realloc(ptr: i32, size: usize) -> *const c_void {
    let mut idx = MALLOC_IDX.lock().unwrap();

    // Since we mutate the vector, the pointer to it can become invalid. Thus, removing it first and re-instering the new pointer later.
    let mut old_vec = match idx.remove(&ptr) {
        Some(v) => v,
        None => panic!("Memory {} not malloced", ptr),
    };
    if size <= old_vec.len() {
        panic!("realloc asked to shrink!"); // TODO
    }
    for _ in 0..size - old_vec.len() {
        old_vec.push(0);
    }

    let p = old_vec.as_ptr();
    idx.insert(p as i32, old_vec);
    crate::log!(
        "reallocing {} bytes. old ptr: {}, new ptr: {}",
        size,
        ptr,
        p as i32
    );
    p as *const c_void
}
