#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
//#![feature(asm)]
mod vga_buffer;
mod serial;

use core::panic::PanicInfo;


#[no_mangle]
pub extern "C" fn _start() {
    #[cfg(test)] //Only compile in test build (not available in general build)
    test_main(); //This function will not return

    println!("Hello");
    println!("World");

    panic!("Some panic message");
    loop {}
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


/* ##################################
   #        Test setup code         #
   ##################################
 */

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_print!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failure);
    loop {}
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
    where T: Fn(),
{
    fn run(&self) -> () {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[OK]");
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        // Creates am IO port and writes to it. The IO port is created by QEMU (See package.metadata.bootimage in Cargo.toml)
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
