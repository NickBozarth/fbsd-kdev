use core::ffi::*;
use crate::types::*;




pub struct cdevsw {
    pub d_open: Option<unsafe extern "C" fn() -> c_int>
}


pub trait Cdev {
    unsafe extern "C" fn open(dev: *mut cdev, oflags: c_int, devtype: c_int, td: *mut thread) -> c_int;
    unsafe extern "C" fn fdopen(dev: *mut cdev, oflags: c_int, td: *mut thread, fp: *mut cfile) -> c_int;
    unsafe extern "C" fn close(dev: *mut cdev, fflag: c_int, devtype: c_int, td: *mut thread) -> c_int;
    unsafe extern "C" fn read(dev: *mut cdev, uio: *mut uio, ioflag: c_int) -> c_int;
    unsafe extern "C" fn write(dev: *mut cdev, uio: *mut uio, ioflag: c_int) -> c_int;
    unsafe extern "C" fn ioctl(dev: *mut cdev, cmd: c_ulong, data: c_caddr_t, fflag: c_int, td: *mut thread) -> c_int;
    unsafe extern "C" fn poll(dev: *mut cdev, events: c_int, td: *mut thread) -> c_int;
    unsafe extern "C" fn mmap(dev: *mut cdev, offset: c_vm_ooffset_t, paddr: *mut c_vm_paddr_t, nprot: c_int, memattr: *mut c_vm_memattr_t) -> c_int;
    unsafe extern "C" fn strategy(bp: *mut bio) -> c_int;
    unsafe extern "C" fn kqfilter(dev: *mut cdev, kn: *mut knote) -> c_int;
    unsafe extern "C" fn purge(dev: *mut cdev) -> c_int;
    unsafe extern "C" fn mmapsingle(cdev: *mut cdev, offset: *mut c_vm_ooffset_t, size: c_vm_size_t, object: *mut *mut vmobject, nprot: c_int) -> c_int;
}
