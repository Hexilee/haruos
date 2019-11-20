const KERNEL_HEAP_SIZE: usize = 0x00a0_0000;
static mut KERNEL_HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
pub const MEMORY_OFFSET: usize = 0x8000_0000;
pub const MEMORY_END: usize = 0x8800_0000;
pub const PAGE_SIZE: usize = 0x1000;
