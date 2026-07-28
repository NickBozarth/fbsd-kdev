use core::ffi::{c_char, c_int};

use crate::driver::device::{Cdevsw, MakeDevArgs};


pub(crate) static mut GLOBAL_CDEVSW: Option<Cdevsw> = None;


unsafe extern "Rust" {
    fn init_dev(
        cdevsw: &mut Option<Cdevsw>, 
        make_dev_args: &mut MakeDevArgs, 
        fmt: &mut *const c_char
    );
}

pub(crate) fn init() -> Result<(), c_int> {
    let mut mda = MakeDevArgs::default();
    let mut fmt: *const c_char = core::ptr::null();
    unsafe { init_dev(&mut GLOBAL_CDEVSW, &mut mda, &mut fmt); }

    if unsafe { GLOBAL_CDEVSW.is_none() } {
        return Err(1); // TODO errno
    }
    if fmt.is_null() {
        return Err(1); // TODO errno
    }

    Ok(())
}
