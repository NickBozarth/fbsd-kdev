







#[allow(non_camel_case_types)]
pub mod typedefs {
    use core::ffi::{c_char, c_int, c_ulong};
    use crate::ffi::device::StructCdev;
    use super::*;

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

    pub type c_uintptr_t = c___uintptr_t;




    pub type DOpenT         = unsafe extern "C" fn(dev: *mut StructCdev, oflags: c_int, devtype: c_int, td: *mut StructThread) -> c_int;
    pub type DFdopenT       = unsafe extern "C" fn(dev: *mut StructCdev, oflags: c_int, td: *mut StructThread, fp: *mut StructCfile) -> c_int;
    pub type DCloseT        = unsafe extern "C" fn(dev: *mut StructCdev, fflag: c_int, devtype: c_int, td: *mut StructThread) -> c_int;
    pub type DReadT         = unsafe extern "C" fn(dev: *mut StructCdev, uio: *mut StructUio, ioflag: c_int) -> c_int;
    pub type DWriteT        = unsafe extern "C" fn(dev: *mut StructCdev, uio: *mut StructUio, ioflag: c_int) -> c_int;
    pub type DIoctlT        = unsafe extern "C" fn(dev: *mut StructCdev, cmd: c_ulong, data: c_caddr_t, fflag: c_int, td: *mut StructThread) -> c_int;
    pub type DPollT         = unsafe extern "C" fn(dev: *mut StructCdev, events: c_int, td: *mut StructThread) -> c_int;
    pub type DMmapT         = unsafe extern "C" fn(dev: *mut StructCdev, offset: c_vm_ooffset_t, paddr: *mut c_vm_paddr_t, nprot: c_int, memattr: *mut c_vm_memattr_t) -> c_int;
    pub type DStrategyT     = unsafe extern "C" fn(bp: *mut StructBio) -> c_int;
    pub type DKqfilterT     = unsafe extern "C" fn(dev: *mut StructCdev, kn: *mut StructKnote) -> c_int;
    pub type DPurgeT        = unsafe extern "C" fn(dev: *mut StructCdev) -> c_int;
    pub type DMmapSingleT   = unsafe extern "C" fn(cdev: *mut StructCdev, offset: *mut c_vm_ooffset_t, size: c_vm_size_t, object: *mut *mut StructVmObject, nprot: c_int) -> c_int;
}




#[repr(C)] pub struct StructThread   { _private: [u8; 0] }
#[repr(C)] pub struct StructCfile    { _private: [u8; 0] }
#[repr(C)] pub struct StructUio      { _private: [u8; 0] }
#[repr(C)] pub struct StructBio      { _private: [u8; 0] }
#[repr(C)] pub struct StructKnote    { _private: [u8; 0] }
#[repr(C)] pub struct StructVmObject { _private: [u8; 0] }
#[repr(C)] pub struct StructUcred    { _private: [u8; 0] }
#[repr(C)] pub struct StructWitness  { _private: [u8; 0] }






