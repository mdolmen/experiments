#![no_std]
#![no_main]

// For custom test framework
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ros::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC] {}", info);

    ros::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::test_panic_handler(info)
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use ros::memory;
    use ros::memory::BootInfoFrameAllocator;
    use x86_64::{
        structures::paging::Page,
        VirtAddr
    };

    // Write through VGA buffer
    //use core::fmt::Write;
    //vga_buffer::WRITER.lock().write_str("Welcome...\n").unwrap();
    //write!(vga_buffer::WRITER.lock(), "...into the MATRIX\n").unwrap();

    println!("Welcome to the MATRIX!");

    // Init IDT
    ros::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    // let addresses = [
    //     // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_info.physical_memory_offset,
    // ];
    //
    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    //let mut frame_allocator = memory::EmptyFrameAllocator;
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(357).write_volatile(0x_f021_f077_f065_f04e)};

    #[cfg(test)]
    test_main();

    ros::hlt_loop();
}
