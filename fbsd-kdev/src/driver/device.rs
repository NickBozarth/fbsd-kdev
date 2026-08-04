use core::ffi::*;
use crate::{cstr, ffi::{device::{StructCdev, StructCdevsw, make_dev_p}, traits::AsCstr, types::{StructUcred, typedefs::*}}};



pub struct Cdevsw(pub(crate) StructCdevsw);


impl Cdevsw {
    pub(crate) const fn new(name: *const c_char) -> Self {
        Self(StructCdevsw::new(name))
    }

    pub const fn with_name(mut self, name: *const c_char) -> Self           { self.0.d_name = name;                   self }
    pub const fn with_flags(mut self, flags: c_int) -> Self                 { self.0.d_flags = flags;                 self }
    pub const fn with_open(mut self, handler: DOpenT) -> Self               { self.0.d_open = Some(handler);          self }
    pub const fn with_fdopen(mut self, handler: DFdopenT) -> Self           { self.0.d_fdopen = Some(handler);        self }
    pub const fn with_close(mut self, handler: DCloseT) -> Self             { self.0.d_close = Some(handler);         self }
    pub const fn with_read(mut self, handler: DReadT) -> Self               { self.0.d_read = Some(handler);          self }
    pub const fn with_write(mut self, handler: DWriteT) -> Self             { self.0.d_write = Some(handler);         self }
    pub const fn with_ioctl(mut self, handler: DIoctlT) -> Self             { self.0.d_ioctl = Some(handler);         self }
    pub const fn with_poll(mut self, handler: DPollT) -> Self               { self.0.d_poll = Some(handler);          self }
    pub const fn with_mmap(mut self, handler: DMmapT) -> Self               { self.0.d_mmap = Some(handler);          self }
    pub const fn with_strategy(mut self, handler: DStrategyT) -> Self       { self.0.d_strategy = Some(handler);      self }
    pub const fn with_kqfilter(mut self, handler: DKqfilterT) -> Self       { self.0.d_kqfilter = Some(handler);      self }
    pub const fn with_purge(mut self, handler: DPurgeT) -> Self             { self.0.d_purge = Some(handler);         self }
    pub const fn with_mmap_single(mut self, handler: DMmapSingleT) -> Self  { self.0.d_mmap_single = Some(handler);   self }
}



pub struct Cdev<'a> {
    _internal: *mut StructCdev, 
    devsw: Cdevsw,
    mda: MakeDevArgs,
    name: &'a str
}

impl Cdev<'_> {
    pub(crate) const fn new() -> Self {
        Self {
            _internal: core::ptr::null_mut(),
            devsw: Cdevsw::new(cstr!("DefaultDevName")),
            mda: MakeDevArgs::default(),
            name: "DefaultDevName"
        }
    }

    pub(crate) fn make_dev(
        &mut self,
    ) -> Result<(), c_int> {


        let cdev = &raw mut self._internal;
        let devsw = &raw mut self.devsw.0;


        let MakeDevArgs { 
            flags, 
            cr, 
            uid, 
            gid, 
            mode 
        } = self.mda;

        let fmt = cstr!("%.*s");

        let result = unsafe {
            make_dev_p(
                flags,
                cdev,
                devsw,
                cr,
                uid,
                gid,
                mode,
                fmt,
                self.name.len() as c_int,
                self.name.as_cstr_unchecked()
            )
        };

        match result {
            0 => Ok(()),
            errno => Err(errno)
        }
    }

    pub(crate) fn set_cdevsw(&mut self, devsw: Cdevsw) { self.devsw = devsw; }
    pub(crate) fn set_mda(&mut self, mda: MakeDevArgs) { self.mda = mda; }
}










#[repr(C)]
pub struct MakeDevArgs {
    flags: c_int,
    cr: *mut StructUcred,
    uid: c_uid_t,
    gid: c_gid_t,
    mode: c_int,
}


impl MakeDevArgs {
    pub const fn default() -> Self {
        Self {
            flags: 0,
            cr: core::ptr::null_mut(),
            uid: 0, // TODO uid consts
            gid: 0, // TODO gid consts
            mode: 0 // TODO mode consts
        }
    }
}
