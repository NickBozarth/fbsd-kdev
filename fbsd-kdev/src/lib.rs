#![no_std]

pub mod prelude;
pub mod ffi;
pub mod driver;
pub mod types;



fn x(y: types::cdev) {}

#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
    loop {}
}
