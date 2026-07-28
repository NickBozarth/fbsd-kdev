use core::ffi::*;

use crate::{cstr, ffi::traits::AsCstr};

unsafe extern "C" {
    fn uprintf(fmt: *const c_char, ...) -> c_int;
}


struct UPrintfWriter;

impl core::fmt::Write for UPrintfWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            uprintf(
                cstr!("%.*s"),
                s.len() as c_int,
                s.as_cstr_unchecked()
            );
        }

        Ok(())
    }
}


pub fn _uprintf(args: core::fmt::Arguments) -> core::fmt::Result {
    let mut writer = UPrintfWriter;
    core::fmt::Write::write_fmt(&mut writer, args)
}
