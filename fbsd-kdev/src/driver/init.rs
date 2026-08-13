use core::ffi::{c_char, c_int};

use crate::{cstr, driver::device::{Cdev, Cdevsw, MakeDevArgs}, sync::mutex::{GlobalMutex, Mutex}};



unsafe extern "Rust" {
    fn init() -> Result<(), c_int>;
}


// TODO could be done with an rw or sx lock or cell
static GLOBAL_CDEV: GlobalMutex<Cdev> = GlobalMutex::new_uninit(cstr!("GLOBAL_CDEV"), Cdev::new);


pub fn set_cdevsw(devsw: Cdevsw) {
    if let Ok(mut lock) = GLOBAL_CDEV.lock() {
        lock.set_cdevsw(devsw);
    }
}

pub fn set_make_dev_args(mda: MakeDevArgs) {
    if let Ok(mut lock) = GLOBAL_CDEV.lock() {
        lock.set_mda(mda);
    }
}

pub(crate) fn init_dev() -> Result<(), c_int> {

    unsafe {
        init()?;
        (&mut *GLOBAL_CDEV.lock().unwrap()).make_dev()?;
    }

    Ok(())
}
