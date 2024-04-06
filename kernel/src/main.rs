//I'll clean up all this comments after implementation, i hate them goes against the principles of keeping code clean.
#![no_std] //disabling std lib.
#![no_main] //overwriting the entry-point

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub extern "C" fn _start() -> ! {
    loop {}
}






