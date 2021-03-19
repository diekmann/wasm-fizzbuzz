// JavaScript imports.

#[link(wasm_import_module = "js")]
extern "C" {
    pub fn console_log(ptr: *const u8, len: usize);
}
