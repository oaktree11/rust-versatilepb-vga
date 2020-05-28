use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::ptr;
use fixed_size_block::FixedSizeBlockAllocator;
#[macro_use]
use crate::print;

/*use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};
*/
pub mod bump;
pub mod fixed_size_block;
pub mod linked_list;

//pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 0x2000000; // 100 KiB

#[global_allocator]
static mut ALLOCATOR: Dummy = Dummy::new();
//static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());
pub struct Dummy{
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}
extern "C" {
    // Boundaries of the .bss section, provided by linker script symbols.
    // static magic: *const c_void;
     #[no_mangle]
     static mut __heap_start :  usize  ;
    // static mut stack_top :  usize  ;
}
pub fn init_heap(){

   unsafe {
        ALLOCATOR.init(&mut __heap_start, HEAP_SIZE);
    }

}
impl Dummy {
    pub const fn new() -> Self {
        Dummy {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }
    /// Initializes the bump allocator with the given heap bounds.
   ///
   /// This method is unsafe because the caller must ensure that the given
   /// memory range is unused. Also, this method must be called only once.
    pub unsafe fn init(&mut self, heap_start: *mut usize, heap_size: usize) {
        print!("alloc init \n");
     //   print!("stack top  {}\n",stack_top);
        ALLOCATOR.heap_start = heap_start as usize;
        print!("alloc init1 {} {} \n", ALLOCATOR.heap_start,99);
        ALLOCATOR.heap_end = ALLOCATOR.heap_start.saturating_add(heap_size);
        ALLOCATOR.next = heap_start as usize;
    }
}
unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
       let mut bump = &mut ALLOCATOR; // get a mutable reference
       print!("alloc called {} {}\n",45,bump.next);
        let alloc_start = bump.next;//align_up(bump.next, layout.align());
        let alloc_end = alloc_start + layout.size();
        /*    match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => {
                print!("return null_mut\n");
                return ptr::null_mut()
            },
        };*/

        if alloc_end > bump.heap_end {
            ptr::null_mut() // out of memory
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            print!("return ptr {}\n",alloc_start);
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = &mut ALLOCATOR; // get a mutable reference

        bump.allocations -= 1;
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
   // unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
     //   print!("alloc called\n");
    //    null_mut()
  //  }

   // unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
    //    panic!("dealloc should be never called")
   // }
}

/// A wrapper around spin::Mutex to permit trait implementations.
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}


/// Align the given address `addr` upwards to alignment `align`.
///
/// Requires that `align` is a power of two.
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
