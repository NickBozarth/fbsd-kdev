#![no_std]

use core::ffi::{c_int, c_void};


pub mod ffi;
pub mod driver;
pub mod types;
pub mod prelude;




#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_modinit(
    _module: *mut c_void,
    event_type: c_int,
    _arg: *mut c_void
) -> c_int {

    match event_type {
        0 => {
            match driver::init::init() {
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
