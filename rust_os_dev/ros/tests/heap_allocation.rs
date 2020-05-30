#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use alloc::{boxed::Box, vec::Vec};
use ros::{serial_print, serial_println, allocator::HEAP_SIZE};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use ros::allocator;
    use ros::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    ros::init();
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_alloc = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_alloc).expect("heap init failed");

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::test_panic_handler(info)
}

#[test_case]
fn simple_allocatino() {
    serial_print!("simple_allocation... ");

    let heap_value_0 = Box::new(100);
    let heap_value_1 = Box::new(200);

    assert_eq!(*heap_value_0, 100);
    assert_eq!(*heap_value_1, 200);

    serial_println!("[ok]");
}

/*
 * Test large and multiple allocations.
 */
#[test_case]
fn large_vec() {
    serial_print!("large_vec... ");

    let n = 1000;
    let mut vec = Vec::new();

    for i in 0..n {
        vec.push(i);
    }

    assert_eq!(vec.iter().sum::<u64>(), (n-1) * n / 2);

    serial_println!("[ok]");
}

/*
 * Test reallocation.
 */
#[test_case]
fn many_boxes() {
    serial_print!("many_boxes... ");

    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }

    serial_println!("[ok]");
}

/*
 * Test reallocation.
 */
#[test_case]
fn many_boxes_long_lived() {
    serial_print!("many_boxes_long_lived... ");

    let long_lived = Box::new(1);
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }

    assert_eq!(*long_lived, 1);
    serial_println!("[ok]");
}
