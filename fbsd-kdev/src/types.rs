use core::ffi::*;





pub type c_size_t       = usize;
pub type c_ssize_t      = isize;
pub type c_uid_t        = u32;
pub type c_gid_t        = u32;
pub type c_caddr_t      = *mut c_char;
pub type c_vm_ooffset_t = u64;
pub type c_vm_memattr_t = c_char;
pub type c_off_t        = u64;

#[cfg(target_pointer_width = "64")]
pub type c_vm_paddr_t = u64;
#[cfg(target_pointer_width = "32")]
pub type c_vm_paddr_t = u32;

#[cfg(target_pointer_width = "64")]
pub type c_vm_size_t = u64;
#[cfg(target_pointer_width = "32")]
pub type c_vm_size_t = u32;

#[cfg(target_pointer_width = "64")]
pub type c___uintptr_t = u64;
#[cfg(target_pointer_width = "32")]
pub type c___uintptr_t = u32;








#[repr(C)]
pub struct thread {
    _private: [u8; 0]
}

#[repr(C)]
pub struct cdev {
    _private: [u8; 0]
}

#[repr(C)]
pub struct cfile {
    _private: [u8; 0]
}

#[repr(C)]
pub struct uio {
    _private: [u8; 0]
}

#[repr(C)]
pub struct bio {
    _private: [u8; 0]
}

#[repr(C)]
pub struct knote {
    _private: [u8; 0]
}

#[repr(C)]
pub struct vmobject {
    _private: [u8; 0]
}









pub type DOpenT         = unsafe extern "C" fn(dev: *mut cdev, oflags: c_int, devtype: c_int, td: *mut thread) -> c_int;
pub type DFdopenT       = unsafe extern "C" fn(dev: *mut cdev, oflags: c_int, td: *mut thread, fp: *mut cfile) -> c_int;
pub type DCloseT        = unsafe extern "C" fn(dev: *mut cdev, fflag: c_int, devtype: c_int, td: *mut thread) -> c_int;
pub type DReadT         = unsafe extern "C" fn(dev: *mut cdev, uio: *mut uio, ioflag: c_int) -> c_int;
pub type DWriteT        = unsafe extern "C" fn(dev: *mut cdev, uio: *mut uio, ioflag: c_int) -> c_int;
pub type DIoctlT        = unsafe extern "C" fn(dev: *mut cdev, cmd: c_ulong, data: c_caddr_t, fflag: c_int, td: *mut thread) -> c_int;
pub type DPollT         = unsafe extern "C" fn(dev: *mut cdev, events: c_int, td: *mut thread) -> c_int;
pub type DMmapT         = unsafe extern "C" fn(dev: *mut cdev, offset: c_vm_ooffset_t, paddr: *mut c_vm_paddr_t, nprot: c_int, memattr: *mut c_vm_memattr_t) -> c_int;
pub type DStrategyT     = unsafe extern "C" fn(bp: *mut bio) -> c_int;
pub type DKqfilterT     = unsafe extern "C" fn(dev: *mut cdev, kn: *mut knote) -> c_int;
pub type DPurgeT        = unsafe extern "C" fn(dev: *mut cdev) -> c_int;
pub type DMmapSingleT   = unsafe extern "C" fn(cdev: *mut cdev, offset: *mut c_vm_ooffset_t, size: c_vm_size_t, object: *mut *mut vmobject, nprot: c_int) -> c_int;
