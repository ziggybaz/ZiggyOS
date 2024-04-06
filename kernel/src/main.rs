//I'll clean up all this comments after implementation, i hate them goes against the principles of keeping code clean.
#![no_std]

use core::panic::PanicInfo;


fn main() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
