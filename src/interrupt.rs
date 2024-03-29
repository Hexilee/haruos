use crate::gdt::DOUBLE_FAULT_IST_INDEX;
use crate::println;
#[cfg(test)]
use crate::test_framework::*;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) {
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint() {
    serial_print!("test breakpoint...");
    x86_64::instructions::interrupts::int3();
    serial_println!("[ok]");
}
