#![no_std]
#![no_main]
#![no_implicit_prelude]

extern crate core;

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(
    r#"
.section .bootsector, "awx"
.global _boot
.code16

_boot:

_infloop:
    jmp _infloop

.org 510
.word 0xaa55
"#
);

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
