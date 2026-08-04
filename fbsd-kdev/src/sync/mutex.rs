use core::cell::UnsafeCell;

use crate::ffi::mutex::StructMutex;


pub enum MutexState {
    Uninitialized,
    Unlocked,
    Locked
}

pub struct Mutex<T> {
    _inner: StructMutex,
    data: UnsafeCell<T>,
}


impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            _inner: StructMutex::new(),
            data: UnsafeCell::new(data)
        }
    }

    pub fn lock() {}
}

// pub struct MutexGuard<T> {
// }
