#[macro_export]
macro_rules! cstr_raw {
    ($s:expr) => ($s.as_ptr() as *const ::core::ffi::c_char)
}

#[macro_export]
macro_rules! cstr {
    ($s:expr) => ($crate::cstr_raw!(::core::concat!($s, "\0")))
}


#[macro_export]
macro_rules! uprint {
    ($($arg:tt)*) => {
        $crate::ffi::io::_uprintf(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! uprintln {
    () => {
        $crate::ffi::io::_uprintf(format_args!("\n"))
    };
    ($($arg:tt)*) => {
        $crate::ffi::io::_uprintf(format_args!("{}\n", format_args!($($arg)*)))
    };
}
