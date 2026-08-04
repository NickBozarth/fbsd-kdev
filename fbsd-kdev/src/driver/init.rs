use core::ffi::{c_char, c_int};

use crate::driver::device::{Cdev, Cdevsw, MakeDevArgs};



unsafe extern "Rust" {
    fn init() -> Result<(), c_int>;
}



static mut GLOBAL_CDEV: Cdev = Cdev::new();


pub fn set_cdevsw(devsw: Cdevsw) {
    unsafe { GLOBAL_CDEV.set_cdevsw(devsw); }
}

pub fn set_make_dev_args(mda: MakeDevArgs) {
    unsafe { GLOBAL_CDEV.set_mda(mda); }
}

pub(crate) fn init_dev() -> Result<(), c_int> {

    unsafe {
        init()?;
        GLOBAL_CDEV.make_dev()?;
    }

    Ok(())
}
