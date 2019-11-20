use buddy_system_allocator::LockedHeap;

const KERNEL_HEAP_SIZE: usize = 0x00a0_0000;
static mut KERNEL_HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
pub const MEMORY_OFFSET: usize = 0x8000_0000;
pub const MEMORY_END: usize = 0x8800_0000;
pub const PAGE_SIZE: usize = 0x1000;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn handle(_: core::alloc::Layout) -> ! {
    panic!("alloc error!")
}
pub fn init(_: usize) {
    unsafe {
        riscv::register::sstatus::set_sum();
    }
}

fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(KERNEL_HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE)
    }
}
