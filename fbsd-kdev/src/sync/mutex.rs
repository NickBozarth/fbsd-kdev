use core::{cell::UnsafeCell, ffi::c_char, ops::{Deref, DerefMut}, sync::atomic::{AtomicBool, Ordering}};
use alloc::boxed::Box;

use crate::ffi::mutex::StructMtx;



pub enum MutexError {
    NotInitialized,
    AlreadyInitialized,
    TryLockFail
}

impl core::fmt::Debug for MutexError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotInitialized     => write!(f, "Mutex is not initialized"),
            Self::AlreadyInitialized => write!(f, "Mutex is already initialized"),
            Self::TryLockFail        => write!(f, "Mutex trylock failed")
        }
    }
}


pub struct Mutex<T> {
    _inner: Box<UnsafeCell<StructMtx>>,
    data:   UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Sync> Sync for Mutex<T> {}


impl<T> Mutex<T> {
    pub fn new(name: *const c_char, data: T) -> Self {
        let mut ret = Self {
            _inner: Box::new(UnsafeCell::new(StructMtx::new())),
            data:   UnsafeCell::new(data),
        };

        unsafe {
            // TODO MTX_DEF const
            ret._inner.get_mut().mtx_init(name, core::ptr::null(), 0x00000000);
        }

        ret
    }

    pub fn lock(&self) -> MutexGuard<T> {
        unsafe { (*self._inner.get()).mtx_lock(); }

        MutexGuard {
            mutex: self
        }
    }

    pub fn try_lock(&self) -> Result<MutexGuard<T>, MutexError> {
        if !self.is_initialized() {
            return Err(MutexError::NotInitialized)
        }

        let ret = unsafe { (*self._inner.get()).mtx_trylock() };

        match ret {
            0 => Err(MutexError::TryLockFail),
            _ => Ok (MutexGuard { mutex: self })
        }
    }

    pub fn is_initialized(&self) -> bool {
        unsafe { (*self._inner.get()).mtx_initialized() }
    }
}

impl<T> Drop for Mutex<T> {
    fn drop(&mut self) {
        unsafe { (&mut *self._inner.get()).mtx_destroy(); }
    }
}



pub struct GlobalMutex<T> {
    _inner: UnsafeCell<Option<Mutex<T>>>,
    is_initialized: AtomicBool,
    name: *const c_char,
    init_fn: fn() -> T
}

unsafe impl<T: Send> Send for GlobalMutex<T> {}
unsafe impl<T: Sync> Sync for GlobalMutex<T> {}

impl<T> GlobalMutex<T> {
    pub const fn new_uninit(name: *const c_char, init_fn: fn() -> T) -> Self {
        Self {
            _inner: UnsafeCell::new(None),
            is_initialized: AtomicBool::new(false),
            name,
            init_fn
        }
    }

    pub fn init(&self) {
        if !self.is_initialized.load(Ordering::Acquire) {
            let data = (self.init_fn)();

            unsafe {
                *self._inner.get() = Some(Mutex::new(self.name, data));
            }

            self.is_initialized.store(true, Ordering::Release);
        }
    }

    pub fn lock(&self) -> Result<MutexGuard<'_, T>, MutexError> {
        if !self.is_initialized.load(Ordering::Acquire) {
            self.init();
        }

        unsafe {
            match &*self._inner.get() {
                Some(mutex) => Ok(mutex.lock()),
                None => Err(MutexError::NotInitialized)
            }
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.is_initialized.load(Ordering::Acquire)
    }
}




pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>
}

unsafe impl<T: Sync> Sync for MutexGuard<'_, T> {}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        unsafe { (*self.mutex._inner.get()).mtx_unlock(); }
    }
}
