pub mod macros;

/*
 * These modules are meant to define unsafe interfaces that will be wrapped by other
 *  safe means (either structs, macros, or functions)
 * */
pub(crate) mod alloc;
pub(crate) mod io;
pub(crate) mod traits;
pub(crate) mod device;
pub(crate) mod types;
pub(crate) mod panic;
pub(crate) mod mutex;
