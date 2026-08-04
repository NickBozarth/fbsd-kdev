use crate::ffi::mutex::StructMutex;

pub struct Mutex<T> {
    _inner: StructMutex,
    data: T,
}
