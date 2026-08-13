/*
 * This is a crate that will be used and changed as an example but
 *  not a permanent one.
 * Dont expect anything super meaningful, just a way to feel how features
 *  might be implemented
 */
#![no_std]

use fbsd_kdev::{driver::init::set_cdevsw, prelude::*, sync::mutex::Mutex};


extern "Rust" fn init() -> Result<(), c_int> {
    let m = Mutex::new(cstr!("test mutex"), 1);
    let mut y = m.lock().unwrap();
    let z = &mut *y;

    set_cdevsw(
        Cdevsw::new(cstr!("example"))
            .with_open()
    );

    Ok(())

}



/*
    struct DOpenParams {
        dev:     *mut StructCdev,
        oflags:  c_int,
        devtype: c_int,
        td:      *mut StructThread
    }

    #[devfunc(open)]
    fn devopen(params: DOpenParams) -> c_int {
        todo!()
    }

-----------------------------------------------
should expand to something like
-----------------------------------------------

    unsafe extern "C" fn devopen(
        dev:     *mut StructCdev,
        oflags:  c_int,
        devtype: c_int,
        td:      *mut StructThread
    ) -> c_int {
        DEVFUNC_devopen(
            DOpenParams {
                dev,
                oflags,
                devtype,
                td
            }
        )
    }

-----------------------------------------------
this is so they are accepted as objects within
the cdevsw struct and can be called by the bsd
kernel
-----------------------------------------------

    Cdev::new(cstr!("devicename"))
        .with_open(devopen)

-----------------------------------------------
and so that interaction with the parameters
will be safer
-----------------------------------------------

    #[devfunc(open)]
    fn devopen(params: DOpenParams) -> c_int {
        let uio: Uio = params.get_uio();
    }

*/
