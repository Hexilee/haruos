pub fn init() {
    crate::gdt::init_gdt();
    crate::interrupt::init_idt();
}
