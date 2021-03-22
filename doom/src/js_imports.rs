// JavaScript imports.

#[link(wasm_import_module = "js")]
extern "C" {
    pub fn js_console_log(ptr: *const u8, len: usize);
    pub fn js_stdout(ptr: *const u8, len: usize);
    pub fn js_stderr(ptr: *const u8, len: usize);
}
