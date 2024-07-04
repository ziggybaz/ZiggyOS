//i'll clean up all this comments after implementation, i hate them goes against the principles of keeping code clean.
#![no_std] //disabling std lib.
#![feature(custom_test_frameworks)]
#![test_runner(operating_system::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_main] //overwriting the entry-point, as std_lib is disabled and we therefore don't have access to both 'Crt0' nor the rust runtime

use operating_system::println;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler] //the first error upon disabling standard library is {1.no panic handler. 2.missing langauge item} //panic handler solution, implementing our own through a function that never returns.
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

//panic handler for test mode, we are using conditional compilation to print to serial port instead of QEMU
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    operating_system::test_panic_handler(info)
}

static OSNAME: &[u8] = b"Ziggy OS Loading";

#[no_mangle] //don't mangle the name of this function instead use 'C' naming convention. also this attribute tells the linker the name of the entry-point
pub extern "C" fn _start() -> ! { //system entry-point, surprised to learn main isn't it in most languages.(named _start by default, standard)
   
    println!("ZiggyOS Initializing. \nBuilt in: {}.", 2024);

    #[cfg(test)]
    test_main();

    loop {}
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

