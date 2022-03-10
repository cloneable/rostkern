#![no_std]
#![no_main]
#![no_implicit_prelude]

extern crate core;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
