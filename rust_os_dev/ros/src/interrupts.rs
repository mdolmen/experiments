use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::println;
use crate::gdt;

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

pub fn idt_init() {
    IDT.load();
}

#[test_case]
fn test_breakpoint_exception() {
    serial_print!("test_breakpoint_exception...");
    x86_64::instructions::interrupts::int3();
    serial_println!("[ok]");
}
