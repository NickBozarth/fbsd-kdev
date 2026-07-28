use core::{ffi::c_char, panic::PanicInfo};

use crate::cstr;

unsafe extern "C" {
    fn panic(fmt: *const c_char, ...) -> !;
}


#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        unsafe {
            panic(
                cstr!("Kernel panic at  %s:%u:%u"),
                location.file().as_ptr() as *const c_char,
                location.line(),
                location.column()
            );
        }
    } else {
        unsafe { panic(cstr!("Kernel panic (unkown location)")); }
    }
}
