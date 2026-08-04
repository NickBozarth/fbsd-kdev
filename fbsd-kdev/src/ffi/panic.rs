use core::ffi::c_char;

unsafe extern "C" {
    pub fn panic(fmt: *const c_char, ...) -> !;
}
