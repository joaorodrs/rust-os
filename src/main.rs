#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle] // ensure that Rust outputs a functions without name cryptography
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
