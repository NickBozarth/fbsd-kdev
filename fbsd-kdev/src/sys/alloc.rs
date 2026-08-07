use core::{alloc::GlobalAlloc, ffi::c_void};

use crate::ffi::alloc::{M_WAITOK, M_ZERO, free, get_malloc_type, malloc_};

struct FreeBSDKalloc;

unsafe impl GlobalAlloc for FreeBSDKalloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe {
            malloc_(layout.size(), get_malloc_type(), M_WAITOK) as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe {
            free(ptr as *mut c_void, get_malloc_type());
        }
    }

    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe {
            malloc_(layout.size(), get_malloc_type(), M_WAITOK | M_ZERO) as *mut u8
        }
    }
}


#[global_allocator]
static KERNELALLOCATOR: FreeBSDKalloc = FreeBSDKalloc;
