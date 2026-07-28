#[macro_export]
macro_rules! cstr_raw {
    ($s:expr) => ($s.as_ptr() as *const ::core::ffi::c_char)
}

#[macro_export]
macro_rules! cstr {
    ($s:expr) => ($crate::cstr_raw!(::core::concat!($s, "\0")))
}

