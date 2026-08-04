use core::{ffi::c_char, fmt::Display};



struct NonNullTerminatedError;
impl Display for NonNullTerminatedError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("AsCstr.as_cstr() failed because data was not null-terminated")
    }
}

pub trait AsCstr {
    fn as_cstr(&self) -> Result<*const c_char, NonNullTerminatedError>;
    fn as_cstr_unchecked(&self) -> *const c_char;
}


impl AsCstr for str {
    fn as_cstr(&self) -> Result<*const c_char, NonNullTerminatedError> {
        self.ends_with('\0')
            .then(|| self.as_ptr() as *const c_char)
            .ok_or_else(|| NonNullTerminatedError)
    }

    fn as_cstr_unchecked(&self) -> *const c_char {
        self.as_ptr() as *const c_char
    }
}

impl AsCstr for [u8] {
    fn as_cstr(&self) -> Result<*const c_char, NonNullTerminatedError> {
        self.last()
            .filter(|&&last_element| last_element == b'\0')
            .map(|_| self.as_ptr() as *const c_char)
            .ok_or_else(|| NonNullTerminatedError)
    }

    fn as_cstr_unchecked(&self) -> *const c_char {
        self.as_ptr() as *const c_char
    }
}
