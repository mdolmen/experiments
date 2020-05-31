use x86_64::structures::idt::{
    InterruptDescriptorTable, InterruptStackFrame,
    PageFaultErrorCode
};
use lazy_static::lazy_static;
use crate::{print, println, gdt, hlt_loop};
use pic8259_simple::ChainedPics;
use spin::Mutex;

pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = Mutex::new(
    unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET) }
);

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

#[cfg(test)]
use crate::{serial_print, serial_println};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(bp_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_int_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_int_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);

        idt
    };
}

extern "x86-interrupt" fn bp_handler(stack_frame: &mut InterruptStackFrame) {
    println!("[EXCEPTION] BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, error: u64) -> !
{
    panic!("[EXCEPTION] DOUBLE FAULT\n{:#?}\nError code: {}", stack_frame, error);
}

extern "x86-interrupt" fn timer_int_handler(_stack_frame: &mut InterruptStackFrame) {
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8()) };
}

extern "x86-interrupt" fn keyboard_int_handler(_stack_frame: &mut InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use x86_64::instructions::port::Port;
    use crate::task::keyboard::add_scancode;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
    }

    let mut keyboard = KEYBOARD.lock();
    // Data port of the PS/2 controller
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    // push scancode to the worker queue
    add_scancode(scancode);

    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8()) };
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame, error: PageFaultErrorCode)
{
    use x86_64::registers::control::Cr2;

    println!("[EXCEPTION] PAGE FAULT");
    println!("Accessed address: {:?}", Cr2::read());
    println!("Error code: {:?}", error);
    println!("{:#?}", stack_frame);
    hlt_loop();
}

pub fn idt_init() {
    IDT.load();
}

#[test_case]
fn test_breakpoint_exception() {
    serial_print!("test_breakpoint_exception...");
    x86_64::instructions::interrupts::int3();
    serial_println!("[ok]");
}
