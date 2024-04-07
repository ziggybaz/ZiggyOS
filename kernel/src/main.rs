//I'll clean up all this comments after implementation, i hate them goes against the principles of keeping code clean.
#![no_std] //disabling std lib.
#![no_main] //overwriting the entry-point

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static OSNAME: &[u88] = b"Ziggy OS Loading";

#[no_mangle] //don't mangle the name of this function instead use 'C' convention
pub extern "C" fn _start() -> ! { //system entry-point, surprised to learn main isn't it in most languages.(named _start by default, standard)
   
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in OSNAME.iter().enumerate() {
        unsafe { //please, this isn't the way we do things in Rust, its only here because rust can't prove the raw pointers created are valid, we use unsafe to calm down the compiler. Don't make a habit of this.
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}






