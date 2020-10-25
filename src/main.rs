use libc::{c_int, size_t};

/*
#[repr(C)]
pub struct MMAllocator {
    // _private: [u8; 0],
    _private: (),
}

extern "C" {
    fn mm_allocator_new(segment_size: u64) -> *const MMAllocator;
}
*/

fn main() {
    println!("making allocator");
    // unsafe {
    //     let alloc = mm_allocator_new(1024);
    // }
    println!("Hello, world!");
}
