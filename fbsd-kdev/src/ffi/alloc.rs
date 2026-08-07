use core::ffi::{c_int, c_void};

use crate::ffi::types::{StructMallocType, typedefs::c_size_t};

unsafe extern "C" {
    pub fn free(addr: *mut c_void, mtype: *mut StructMallocType);

    fn malloc(
        size: c_size_t, 
        mtype: *mut StructMallocType, 
        flags: c_int
    ) -> *mut c_void;

    pub fn get_malloc_type() -> *mut StructMallocType;
}


pub const M_WAITOK: c_int = 0x0002;
pub const M_ZERO: c_int = 0x0100; // TODO consts


#[must_use]
pub unsafe fn malloc_(
    size: c_size_t,
    mtype: *mut StructMallocType,
    flags: c_int
) -> *mut c_void {
    if flags & M_ZERO != 0 {
        let buf = unsafe {
            malloc(size, mtype, flags)
        };

        if !buf.is_null() {
            unsafe {
                core::ptr::write_bytes(
                    buf,
                    0,
                    size
                ); 
            }
        }
        buf
    } else {
        unsafe { malloc(size, mtype, flags) }
    }
}
