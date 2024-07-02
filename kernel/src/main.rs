//i'll clean up all this comments after implementation, i hate them goes against the principles of keeping code clean.
#![no_std] //disabling std lib.
#![no_main] //overwriting the entry-point

mod vga_buffer;

use core::panic::PanicInfo;
use core::fmt::Write;

#[panic_handler] //the first error upon disabling standard library is {1.no panic handler. 2.missing langauge item}
//panic handler solution, implementing our own through a function that never returns
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

static OSNAME: &[u8] = b"Ziggy OS Loading";

#[no_mangle] //don't mangle the name of this function instead use 'C' calling convention
pub extern "C" fn _start() -> ! { //system entry-point, surprised to learn main isn't it in most languages.(named _start by default, standard)
   
    println!("ZiggyOS Initializing. \nBuilt in: {}.", 2024);

    loop {}
}



#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
