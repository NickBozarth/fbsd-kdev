use core::ffi::{c_char, c_int, c_uint};

use crate::{cstr, ffi::{types::{StructWitness, typedefs::{c___uintptr_t, c_uintptr_t}}}};


/*
 * SUPPORTED FUNCTIONS SO FAR
 *
 * mtx_init()
 * mtx_lock()
 * mtx_unlock()
 * mtx_destroy()
 * */


unsafe extern "C" {
    fn _mtx_init(
        c:      *mut c___uintptr_t,
        name:   *const c_char,
        mtype:  *const c_char,
        opts:   c_int
    );

    fn __mtx_lock_flags(
        c: *mut c_uintptr_t,
        opts: c_int,
        file: *const c_char,
        line: c_int
    );

    fn _mtx_trylock_flags_(
        c: *mut c_uintptr_t,
        opts: c_int,
        file: *const c_char,
        line: c_int
    ) -> c_int;

    fn __mtx_unlock_flags(
        c: *mut c_uintptr_t,
        opts: c_int,
        file: *const c_char,
        line: c_int
    );

    fn _mtx_destroy(c: *mut c_uintptr_t);
}



impl StructMtx {

// #define	mtx_init(m, n, t, o)						\
// 	_mtx_init(&(m)->mtx_lock, n, t, o)
    pub unsafe fn mtx_init(
        &mut self,
        n: *const c_char,
        t: *const c_char,
        o: c_int
    ) {
        let c = &raw mut self.mtx_lock;
        let name = n;
        let mtype = t;
        let opts = o;
        unsafe { _mtx_init(c, name, mtype, opts); }
    }

// #define	_mtx_lock_flags(m, o, f, l)					\
// 	__mtx_lock_flags(&(m)->mtx_lock, o, f, l)
    unsafe fn _mtx_lock_flags(
        &mut self,
        o: c_int,
        f: *const c_char,
        l: c_int
    ) {
        unsafe {
            __mtx_lock_flags(
                &raw mut self.mtx_lock,
                o,
                f,
                l
            );
        }
    }
// #define	mtx_lock_flags_(m, opts, file, line)				\
// 	_mtx_lock_flags((m), (opts), (file), (line))
    unsafe fn mtx_lock_flags_(
        &mut self,
        opts: c_int,
        file: *const c_char,
        line: c_int
    ) {
        unsafe {
            self._mtx_lock_flags(opts, file, line);
        }
    }
// #define	mtx_lock_flags(m, opts)						\
// 	mtx_lock_flags_((m), (opts), LOCK_FILE, LOCK_LINE)
    unsafe fn mtx_lock_flags(&mut self, opts: c_int) {
        unsafe {
            self.mtx_lock_flags_(
                opts,
                cstr!(file!()),
                line!() as c_int
            );
        }
    }
// #define mtx_lock(m)		mtx_lock_flags((m), 0)
    pub unsafe fn mtx_lock(&mut self) {
        unsafe { self.mtx_lock_flags(0); }
    }



// #define	mtx_trylock_flags_(m, o, f, l)					\
// 	_mtx_trylock_flags_(&(m)->mtx_lock, o, f, l)
    unsafe fn mtx_trylock_flags_(
        &mut self,
        o: c_int,
        f: *const c_char,
        l: c_int
    ) -> c_int {
        unsafe {
            _mtx_trylock_flags_(
                &raw mut self.mtx_lock,
                o,
                f,
                l
            )
        }
    }
// #define mtx_trylock_flags(m, opts)					\
// 	mtx_trylock_flags_((m), (opts), LOCK_FILE, LOCK_LINE)
    unsafe fn mtx_trylock_flags(&mut self, opts: c_int) -> c_int {
        unsafe {
            self.mtx_trylock_flags_(
                opts,
                cstr!(file!()),
                line!() as c_int
            )
        }
    }
// #define mtx_trylock(m)		mtx_trylock_flags((m), 0)
    pub fn mtx_trylock(&mut self) -> c_int {
        unsafe { self.mtx_trylock_flags(0) }
    }





// #define	_mtx_unlock_flags(m, o, f, l)					\
// 	__mtx_unlock_flags(&(m)->mtx_lock, o, f, l)
    unsafe fn _mtx_unlock_flags(
        &mut self,
        o: c_int,
        f: *const c_char,
        l: c_int
    ) {
        unsafe {
            __mtx_unlock_flags(
                &raw mut self.mtx_lock,
                o,
                f,
                l
            );
        }
    }
// #define	mtx_unlock_flags_(m, opts, file, line)				\
// 	_mtx_unlock_flags((m), (opts), (file), (line))
    unsafe fn mtx_unlock_flags_(
        &mut self,
        opts: c_int,
        file: *const c_char,
        line: c_int
    ) {
        unsafe {
            self._mtx_unlock_flags(opts, file, line);
        }
    }
// #define	mtx_unlock_flags(m, opts)					\
// 	mtx_unlock_flags_((m), (opts), LOCK_FILE, LOCK_LINE)
    unsafe fn mtx_unlock_flags(&mut self, opts: c_int) {
        unsafe {
            self.mtx_unlock_flags_(
                opts,
                cstr!(file!()),
                line!() as c_int
            );
        }
    }
// #define mtx_unlock(m)		mtx_unlock_flags((m), 0)
    pub unsafe fn mtx_unlock(&mut self) {
        unsafe { self.mtx_unlock_flags(0); }
    }




// #define	mtx_destroy(m)							\
// 	_mtx_destroy(&(m)->mtx_lock)
    pub unsafe fn mtx_destroy(&mut self) {
        unsafe { _mtx_destroy(&raw mut self.mtx_lock); }
    }

    pub fn mtx_initialized(&self) -> bool {
        self.lock_object.lock_initialized()
    }
}




#[repr(C)]
pub struct StructMtx {
    lock_object: StructLockObject,
    mtx_lock: c___uintptr_t
}

impl StructMtx {
    pub const fn new() -> Self {
        Self {
            lock_object:    StructLockObject::new(),
            mtx_lock:       0x04
        }
    }
}



#[repr(C)]
pub struct StructLockObject {
    lo_name: *const c_char,
    lo_flags: c_uint,
    lo_data: c_uint,
    lo_witness: *mut StructWitness
}

impl StructLockObject {
    pub const fn new() -> Self {
        Self {
            lo_name: core::ptr::null(),
            lo_flags: 0,
            lo_data: 0,
            lo_witness: core::ptr::null_mut()
        }
    }

    pub fn lock_initialized(&self) -> bool {
        (self.lo_flags & 0x00010000) != 0
    }
}
