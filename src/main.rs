#![no_std]
#![no_main]
#![no_implicit_prelude]
#![feature(asm_const)]

extern crate core;

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(
    r#"
.section .bootsector, "awx"
.global _boot
.code16

_boot:

_protected_mode:
    cli
    lgdt [_gdt_hdr]
    mov eax, cr0
    or al, 1
    mov cr0, eax

_infloop:
    jmp _infloop

_gdt_hdr:
    .word _gdt_end - _gdt_start - 1
    .word _gdt_start
_gdt_start:
    .quad 0
    .quad {codesegment}
    .quad {datasegment}
_gdt_end:

    .org 510
    .word 0xaa55
"#,
codesegment = const gdt_entry_32(0, 0xFFFFF, 0b10011010, 0b1100),
datasegment = const gdt_entry_32(0, 0xFFFFF, 0b10010010, 0b1100),
);

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

const fn gdt_entry_32(base: u32, limit: u32, access: u8, flags: u8) -> u64 {
    let mut entry: u64 = limit as u64 & 0xFFFF;
    entry |= (limit as u64 & 0xF0000) << 32;
    entry |= (base as u64 & 0xFFFFFF) << 16;
    entry |= (base as u64 & 0xFF000000) << 32;
    entry |= (access as u64) << 40;
    entry |= (flags as u64) << 52;
    entry
}
