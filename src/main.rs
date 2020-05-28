// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>

// Rust embedded logo for `make doc`.
#![doc(html_logo_url = "https://git.io/JeGIp")]

//! The `kernel` binary.
//!
//! # Code organization and architecture
//!
//! The code is divided into different *modules*, each representing a typical **subsystem** of the
//! `kernel`. Top-level module files of subsystems reside directly in the `src` folder. For example,
//! `src/memory.rs` contains code that is concerned with all things memory management.
//!
//! ## Visibility of processor architecture code
//!
//! Some of the `kernel`'s subsystems depend on low-level code that is specific to the target
//! processor architecture. For each supported processor architecture, there exists a subfolder in
//! `src/_arch`, for example, `src/_arch/aarch64`.
//!
//! The architecture folders mirror the subsystem modules laid out in `src`. For example,
//! architectural code that belongs to the `kernel`'s memory subsystem (`src/memory.rs`) would go
//! into `src/_arch/aarch64/memory.rs`. The latter file is directly included and re-exported in
//! `src/memory.rs`, so that the architectural code parts are transparent with respect to the code's
//! module organization. That means a public function `foo()` defined in
//! `src/_arch/aarch64/memory.rs` would be reachable as `crate::memory::foo()` only.
//!
//! The `_` in `_arch` denotes that this folder is not part of the standard module hierarchy.
//! Rather, it's contents are conditionally pulled into respective files using the `#[path =
//! "_arch/xxx/yyy.rs"]` attribute.
//!
//! ## BSP code
//!
//! `BSP` stands for Board Support Package. `BSP` code is organized under `src/bsp.rs` and contains
//! target board specific definitions and functions. These are things such as the board's memory map
//! or instances of drivers for devices that are featured on the respective board.
//!
//! Just like processor architecture code, the `BSP` code's module structure tries to mirror the
//! `kernel`'s subsystem modules, but there is no transparent re-exporting this time. That means
//! whatever is provided must be called starting from the `bsp` namespace, e.g.
//! `bsp::driver::driver_manager()`.
//!
//! ## Kernel interfaces
//!
//! Both `arch` and `bsp` contain code that is conditionally compiled depending on the actual target
//! and board for which the kernel is compiled. For example, the `interrupt controller` hardware of
//! the `Raspberry Pi 3` and the `Raspberry Pi 4` is different, but we want the rest of the `kernel`
//! code to play nicely with any of the two without much hassle.
//!
//! In order to provide a clean abstraction between `arch`, `bsp` and `generic kernel code`,
//! `interface` traits are provided *whenever possible* and *where it makes sense*. They are defined
//! in the respective subsystem module and help to enforce the idiom of *program to an interface,
//! not an implementation*. For example, there will be a common IRQ handling interface which the two
//! different interrupt controller `drivers` of both Raspberrys will implement, and only export the
//! interface to the rest of the `kernel`.
//!
//! ```
//!         +-------------------+
//!         | Interface (Trait) |
//!         |                   |
//!         +--+-------------+--+
//!            ^             ^
//!            |             |
//!            |             |
//! +----------+--+       +--+----------+
//! | kernel code |       |  bsp code   |
//! |             |       |  arch code  |
//! +-------------+       +-------------+
//! ```
//!
//! # Summary
//!
//! For a logical `kernel` subsystem, corresponding code can be distributed over several physical
//! locations. Here is an example for the **memory** subsystem:
//!
//! - `src/memory.rs` and `src/memory/**/*`
//!   - Common code that is agnostic of target processor architecture and `BSP` characteristics.
//!     - Example: A function to zero a chunk of memory.
//!   - Interfaces for the memory subsystem that are implemented by `arch` or `BSP` code.
//!     - Example: An `MMU` interface that defines `MMU` function prototypes.
//! - `src/bsp/__board_name__/memory.rs` and `src/bsp/__board_name__/memory/**/*`
//!   - `BSP` specific code.
//!   - Example: The board's memory map (physical addresses of DRAM and MMIO devices).
//! - `src/_arch/__arch_name__/memory.rs` and `src/_arch/__arch_name__/memory/**/*`
//!   - Processor architecture specific code.
//!   - Example: Implementation of the `MMU` interface for the `__arch_name__` processor
//!     architecture.
//!
//! From a namespace perspective, **memory** subsystem code lives in:
//!
//! - `crate::memory::*`
//! - `crate::bsp::memory::*`

#![no_main]
#![no_std]
#![feature(asm)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(alloc_layout_extra)]
#![feature(const_fn)]
#![feature(const_in_array_repeat_expressions)]
#![feature(core_intrinsics)]

#[allow(dead_code)]

#[macro_use]
extern crate alloc;

// `mod cpu` provides the `_start()` function, the first function to run. `_start()` then calls
// `runtime_init()`, which jumps to `kernel_init()`.
//extern crate compiler_builtins;
mod bsp;
mod console;
mod cpu;
mod memory;
mod panic_wait;
pub mod print;
mod runtime_init;
mod display;
mod framebuffer;
mod output_settings;
mod theme;
pub mod allocator;
mod string;
mod intrinsics;
mod vga;
mod window;

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::{PrimitiveStyle, TextStyle},
};
use alloc::{boxed::Box};
use crate::vga::Vga;
use crate::theme::BinaryColorTheme;
use crate::output_settings::OutputSettingsBuilder;
use crate::display::SimulatorDisplay;
use crate::window::Window;

fn main1()   {
    // Create a new monochrome simulator display with 128x64 pixels.
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(640, 480));

    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);
    let text_style = TextStyle::new(Font6x8, BinaryColor::On);

    let yoffset = 10;

    // Draw a 3px wide outline around the display.
    let bottom_right = Point::zero() + display.size() - Point::new(1, 1);
    Rectangle::new(Point::zero(), bottom_right)
        .into_styled(thick_stroke)
        .draw(&mut display);

    // Draw a triangle.
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(thin_stroke)
    .draw(&mut display);

    // Draw a filled square
    Rectangle::new(Point::new(52, yoffset), Point::new(52 + 16, 16 + yoffset))
        .into_styled(fill)
        .draw(&mut display);

    // Draw a circle with a 3px wide stroke.
    Circle::new(Point::new(88, yoffset), 17)
        .into_styled(thick_stroke)
        .draw(&mut display);

    // Draw centered text.
    //let text = "embedded-graphics";
    //let width = text.len() as i32 * 6;
   // Text::new(text, Point::new(64 - width / 2, 40))
    //    .into_styled(text_style)
     //   .draw(&mut display);
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut w = Window::new( &output_settings);
    w.update(&display);
   
}
/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.

unsafe fn kernel_init(){
    extern "C" {
        // Boundaries of the .bss section, provided by linker script symbols.
        // static magic: *const c_void;
        #[no_mangle]
        static mut __heap_start :  usize  ;
        static mut __bss_start: usize;
        static mut __bss_end: usize;

    }
    let y = &__heap_start;
    let y1 = &__bss_start;
    println!("[0] Hello from Rust!");
    println!("[0] heap {}",y);
    println!("[0] bss {}",y1);

    allocator::init_heap();
    let x = 65;
    let heap_value1 = &x;
    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value1);
    println!("heap_value at {:p}", heap_value);
    println!("heap_value at {}", *heap_value);
    let mut vga : Vga = Vga::new();
    Vga::init();
    main1();
    loop{

    }
}



#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
