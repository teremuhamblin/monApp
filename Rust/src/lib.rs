#[no_mangle]
pub extern "C" fn rust_hello() -> *const i8 {
    use std::ffi::CString;
    CString::new("Bonjour depuis Rust !").unwrap().into_raw()
}
