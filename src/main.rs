#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

static HELLO: &[u8] = b"deez OS";

#[no_mangle] // Don't mangle the name of this function, Linux calling conventions look for a _start
// entry point by defualt
pub extern "C" fn _start() -> ! {
    println!("Hello {}", "world");
    panic!("panik baby");
    loop {}
}
