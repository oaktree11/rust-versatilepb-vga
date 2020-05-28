## SPDX-License-Identifier: MIT OR Apache-2.0
##
## Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>

# Default to the RPi3
BSP ?= versatilepb

# BSP-specific arguments
ifeq ($(BSP),versatilepb)
    TARGET            = armv5te-unknown-linux-gnueabi
#    TARGET            = thumbv7m-none-eabi
    KERNEL_BIN        = kernel8.img
#    QEMU_BINARY       = qemu-system-aarch64
    QEMU_BINARY       = qemu-system-arm
    QEMU_MACHINE_TYPE = versatilepb
    QEMU_RELEASE_ARGS = -serial mon:stdio
    LINKER_FILE       = src/bsp/versatilepb/link.ld
    RUSTC_MISC_ARGS   = -C link-arg=-fno-builtin-bcmp -C link-arg=-nostartfiles -C linker=arm-none-eabi-gcc -C target-cpu=arm926ej-s
else ifeq ($(BSP),rpi4)
    TARGET            = aarch64-unknown-none-softfloat
    KERNEL_BIN        = kernel8.img
    QEMU_BINARY       = qemu-system-aarch64
    QEMU_MACHINE_TYPE =
    QEMU_RELEASE_ARGS = -serial mon:stdio
    LINKER_FILE       = src/bsp/raspberrypi/link.ld
    RUSTC_MISC_ARGS   = -C target-cpu=cortex-a72
endif

# Export for build.rs
export LINKER_FILE

#RUSTFLAGS          = +nightly -C link-arg=-T$(LINKER_FILE) $(RUSTC_MISC_ARGS)
RUSTFLAGS          = -C link-arg=-T$(LINKER_FILE) $(RUSTC_MISC_ARGS)
RUSTFLAGS_PEDANTIC = $(RUSTFLAGS) 

COMPILER_ARGS = --target=$(TARGET) \
    --features bsp_$(BSP)          
  

#RUSTC_CMD   = cargo +nightly rustc $(COMPILER_ARGS)
RUSTC_CMD   = cargo rustc $(COMPILER_ARGS)
DOC_CMD     = cargo doc $(COMPILER_ARGS)
CLIPPY_CMD  = cargo clippy $(COMPILER_ARGS)
CHECK_CMD   = cargo check $(COMPILER_ARGS)
OBJCOPY_CMD = rust-objcopy \
    --strip-all            \
    -O binary

KERNEL_ELF = target/$(TARGET)/debug/kernel

DOCKER_IMAGE         = rustembedded/osdev-utils
DOCKER_CMD           = docker run -it --rm -v $(shell pwd):/work/tutorial -w /work/tutorial

#DOCKER_QEMU = $(DOCKER_CMD) $(DOCKER_IMAGE)

EXEC_QEMU = $(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE)

.PHONY: all $(KERNEL_ELF) $(KERNEL_BIN) doc qemu clippy clean readelf objdump nm check

all: $(KERNEL_BIN)

$(KERNEL_ELF):
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)

$(KERNEL_BIN): $(KERNEL_ELF)
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)

doc:
	$(DOC_CMD) --document-private-items --open

ifeq ($(QEMU_MACHINE_TYPE),)
qemu:
	@echo "This board is not yet supported for QEMU."
else
qemu: $(KERNEL_BIN)
	@$(DOCKER_QEMU) $(EXEC_QEMU) $(QEMU_RELEASE_ARGS) -kernel $(KERNEL_BIN)
endif

clippy:
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(CLIPPY_CMD)

clean:
	rm -rf target $(KERNEL_BIN)

readelf: $(KERNEL_ELF)
	readelf -a $(KERNEL_ELF)

objdump: $(KERNEL_ELF)
	rust-objdump --arch-name aarch64 --disassemble --demangle --no-show-raw-insn \
	    --print-imm-hex $(KERNEL_ELF)

nm: $(KERNEL_ELF)
	rust-nm --demangle --print-size $(KERNEL_ELF) | sort

# For rust-analyzer
check:
	@RUSTFLAGS="$(RUSTFLAGS)" $(CHECK_CMD) --message-format=json
