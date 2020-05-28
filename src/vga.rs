use core::ptr::write_volatile;
pub struct Vga;
impl Vga {
    pub const fn new() -> Self {
        Vga {}
    }
    /// Initializes the bump allocator with the given heap bounds.
   ///
   /// This method is unsafe because the caller must ensure that the given
   /// memory range is unused. Also, this method must be called only once.
    pub unsafe fn init() {
        unsafe {//0x101f1000
            // core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
            //core::ptr::write_volatile(0x101f1000 as *mut u32, (c as u8).into());

            let mut fb = 0x200000 as *mut u32;

            /********* for 640x480 ************************/
            write_volatile(0x1000001c as *mut u32, 0x2C77);
            write_volatile(0x10120000 as *mut u32, 0x3F1F3F9C);
            write_volatile(0x10120004 as *mut u32, 0x090B61DF);
            write_volatile(0x10120008 as *mut u32, 0x067F1800);
            write_volatile(0x10120010 as *mut u32, 0x200000);
            write_volatile(0x10120018 as *mut u32, 0x82B);

            //cursor = 127; // cursor bit map in font0 at 127
            let x = 0;
            for x in 0..(640 * 480) {
                *fb.offset(x) = 0x77;   // black screen
            }
        }
    }

    pub unsafe fn setPointer(ptr: *const u8) {
        unsafe {
            write_volatile(0x10120010 as *mut u32, ptr as u32);
        }
    }
}