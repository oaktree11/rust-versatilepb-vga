# rust-versatilepb-vga
using rust to build an embedded firmware for the versatilepb board.

embedded_graphics crate ported to the versatilepb board.
https://docs.rs/embedded-graphics/0.6.2/embedded_graphics/

make

make qemu

Code modified from here https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials

need to install armv5te-unknown-linux-gnueabi using rustup

hacked intrinsics.rs 
__kuser_cmpxchg
__kuser_memory_barrier
getting called beacuse the compiler (armv5te-unknown-linux-gnueabi) expects a linux OS see here
https://www.kernel.org/doc/Documentation/arm/kernel_user_helpers.txt

Could not find an easy way of generating correct code for the arm926ej-s machine except by using this toolchain.
