#![no_std]
#![no_main]

use core::panic::PanicInfo;
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

mod vga_buffer;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    println!("Hello, world!");
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap(); 
    loop {}
}

