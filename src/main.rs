#![no_std]
#![no_main]
//#![feature(asm)]
mod vga_buffer;

use core::panic::PanicInfo;


#[no_mangle]
pub extern "C" fn _start() {
    /*use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), "some numbers {} and {}", 42, 1.337).unwrap();*/

    println!("Hello");
    println!("World");

    panic!("Some panic message");
    loop {}
}



#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
