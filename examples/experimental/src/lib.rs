/*
 * This is a crate that will be used and changed as an example but
 *  not a permanent one.
 * Dont expect anything super meaningful, just a way to feel how features
 *  might be implemented
 */
#![no_std]

use fbsd_kdev::{prelude::*, sync::mutex::Mutex};


extern "Rust" fn init() -> Result<(), c_int> {
    let m = Mutex::new(cstr!("test mutex"), 1);
    let mut y = m.lock().unwrap();
    let z = &mut *y;

    Ok(())

}


struct DOpenParams;

fn open(params: DOpenParams) -> c_int {
    todo!()
}
