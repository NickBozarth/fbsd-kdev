use core::ffi::{c_char, c_int, c_void};

use crate::ffi::types::*;
use crate::ffi::types::typedefs::*;

unsafe extern "C" {
    pub(crate) unsafe fn make_dev_p(
        flags:  c_int,
        cdev:   *mut *mut StructCdev,
        devsw:  *mut StructCdevsw,
        cr:     *mut StructUcred,
        uid:    c_uid_t,
        gid:    c_gid_t,
        mode:   c_int,
        fmt:    *const c_char,
        ...
    ) -> c_int;
}



pub mod driver_version {
    use super::*;

    pub const V00: c_int     = 0x20011966;
    pub const V01: c_int     = 0x17032005;
    pub const V02: c_int     = 0x28042009;
    pub const V03: c_int     = 0x17122009;
    pub const V04: c_int     = 0x5c48c353;
    pub const CURRENT: c_int = V04;
}


#[repr(C)] pub struct StructCdev     { _private: [u8; 0] }


#[repr(C)]
pub struct StructCdevsw {
    pub d_version:      c_int,
    pub d_flags:        c_int,
    pub d_name:         *const c_char,
    pub d_open:         Option<DOpenT>,
    pub d_fdopen:       Option<DFdopenT>,
    pub d_close:        Option<DCloseT>,
    pub d_read:         Option<DReadT>,
    pub d_write:        Option<DWriteT>,
    pub d_ioctl:        Option<DIoctlT>,
    pub d_poll:         Option<DPollT>,
    pub d_mmap:         Option<DMmapT>,
    pub d_strategy:     Option<DStrategyT>,
    pub d_spare0:       *mut c_void,
    pub d_kqfilter:     Option<DKqfilterT>,
    pub d_purge:        Option<DPurgeT>,
    pub d_mmap_single:  Option<DMmapSingleT>,
    
    pub d_spare1:       [i32; 3],
    pub d_spare2:       [*mut c_void; 3],

    /* These fields should not be messed with by drivers */
    d_devs_lh:          *mut c_void,
    d_spare3:           c_int,
    __d_giant:          *mut c_void,
}

impl StructCdevsw {
    pub const fn new(name: *const c_char) -> Self {
        Self {
            d_version: driver_version::CURRENT,
            d_flags: 0,
            d_name: name,
            d_open: None,
            d_fdopen: None,
            d_close: None,
            d_read: None,
            d_write: None,
            d_ioctl: None,
            d_poll: None,
            d_mmap: None,
            d_strategy: None,
            d_spare0: ::core::ptr::null_mut(),
            d_kqfilter: None,
            d_purge: None,
            d_mmap_single: None,

            d_spare1: [0; 3],
            d_spare2: [core::ptr::null_mut(); 3],

            d_devs_lh: ::core::ptr::null_mut(),
            d_spare3: 0,
            __d_giant: ::core::ptr::null_mut(),
        }
    }
}
