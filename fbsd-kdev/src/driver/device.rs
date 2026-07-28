use core::ffi::*;
use crate::types::{c_size_t, *};



unsafe extern "C" {
    fn make_dev_args_init_impl(
        _args: *mut MakeDevArgs, 
        _sz: c_size_t
    );
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




#[repr(C)]
pub struct Cdevsw {
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


impl Cdevsw {
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

    pub const fn with_flags(mut self, flags: c_int) -> Self         { self.d_flags = flags;                 self }

    pub const fn with_open(mut self, handler: DOpenT) -> Self               { self.d_open = Some(handler);          self }
    pub const fn with_fdopen(mut self, handler: DFdopenT) -> Self           { self.d_fdopen = Some(handler);        self }
    pub const fn with_close(mut self, handler: DCloseT) -> Self             { self.d_close = Some(handler);         self }
    pub const fn with_read(mut self, handler: DReadT) -> Self               { self.d_read = Some(handler);          self }
    pub const fn with_write(mut self, handler: DWriteT) -> Self             { self.d_write = Some(handler);         self }
    pub const fn with_ioctl(mut self, handler: DIoctlT) -> Self             { self.d_ioctl = Some(handler);         self }
    pub const fn with_poll(mut self, handler: DPollT) -> Self               { self.d_poll = Some(handler);          self }
    pub const fn with_mmap(mut self, handler: DMmapT) -> Self               { self.d_mmap = Some(handler);          self }
    pub const fn with_strategy(mut self, handler: DStrategyT) -> Self       { self.d_strategy = Some(handler);      self }
    pub const fn with_kqfilter(mut self, handler: DKqfilterT) -> Self       { self.d_kqfilter = Some(handler);      self }
    pub const fn with_purge(mut self, handler: DPurgeT) -> Self             { self.d_purge = Some(handler);         self }
    pub const fn with_mmap_single(mut self, handler: DMmapSingleT) -> Self  { self.d_mmap_single = Some(handler);   self }
}

unsafe impl Send for Cdevsw {}





#[repr(C)]
pub struct MakeDevArgs {
    mda_size: c_size_t,
    pub mda_flags: c_int,
    pub mda_devsw: *mut Cdevsw,
    pub mda_cr: *mut ucred,
    pub mda_uid: c_uid_t,
    pub mda_gid: c_gid_t,
    pub mda_mode: c_int,
    pub mda_uint: c_int,
    pub mda_si_drv1: *mut c_void,
    pub mda_si_drv2: *mut c_void,
}


impl MakeDevArgs {
    pub fn default() -> Self {
        /*
         * All args are safe to be zeroed before initialization
         */
        let mut args: Self;
        unsafe { 
            args = ::core::mem::zeroed();
            make_dev_args_init_impl(&raw mut args, ::core::mem::size_of::<Self>());
        };

        args
    }
}
