/*
 * This is a crate that will be used and changed as an example but
 *  not a permanent one.
 * Dont expect anything super meaningful, just a way to feel how features
 *  might be implemented
 */
#![no_std]

use fbsd_kdev::driver::device::Cdev;

struct MyDev;

impl Cdev for MyDev {
}
