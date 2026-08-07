#![no_std]

use core::ffi::{c_int, c_void};

extern crate alloc;

pub mod ffi;
pub mod driver;
pub mod types;
pub mod prelude;
pub mod sys;
pub mod sync;



#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_modinit(
    _module: *mut c_void,
    event_type: c_int,
    _arg: *mut c_void
) -> c_int {

    match event_type {
        0 => {
            match driver::init::init_dev() {
                Ok(()) => {
                    uprintln!("Successfully created device");
                    0
                },
                Err(err) => err
            }
        }

        1 => {
            0
        }

        _ => {
            1
        }
    }

}
