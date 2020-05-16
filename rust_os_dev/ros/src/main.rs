#![no_std]
#![no_main]

// For custom test framework
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ros::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC] {}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Write through VGA buffer
    //use core::fmt::Write;
    //vga_buffer::WRITER.lock().write_str("Welcome...\n").unwrap();
    //write!(vga_buffer::WRITER.lock(), "...into the MATRIX\n").unwrap();

    println!("Welcome to the MATRIX!");
    //panic!("test");

    // Init IDT and trigger a breakpoint
    ros::init();
    x86_64::instructions::interrupts::int3();

    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

    #[cfg(test)]
    test_main();

    loop {}
}
