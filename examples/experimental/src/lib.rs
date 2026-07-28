/*
 * This is a crate that will be used and changed as an example but
 *  not a permanent one.
 * Dont expect anything super meaningful, just a way to feel how features
 *  might be implemented
 */
#![no_std]

use core::ffi::*;
use fbsd_kdev::prelude::*;

unsafe extern "C" fn d_open(
    dev: *mut cdev, 
    oflags: c_int, 
    devtype: c_int, 
    td: *mut thread
) -> c_int {
    0
}

const EXPERIMENTAL_CDEVSW: Cdevsw = Cdevsw::new(cstr!("experimental"))
    .with_open(d_open);

fn init_dev(global_cdev: &mut Option<Cdevsw>) {
    *global_cdev = Some(EXPERIMENTAL_CDEVSW);
}
