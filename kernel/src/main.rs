//i'll clean up all this comments after implementation, i hate them goes against the principles of keeping code clean.
#![no_std] //disabling std lib.
#![no_main] //overwriting the entry-point
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

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
    serial_println!("[FAILED]\n");
    serial_println!("ERROR: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    loop{}
}

static OSNAME: &[u8] = b"Ziggy OS Loading";

#[no_mangle] //don't mangle the name of this function instead use 'C' calling convention
pub extern "C" fn _start() -> ! { //system entry-point, surprised to learn main isn't it in most languages.(named _start by default, standard)
   
    println!("ZiggyOS Initializing. \nBuilt in: {}.", 2024);

    #[cfg(test)]
    test_main();

    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

