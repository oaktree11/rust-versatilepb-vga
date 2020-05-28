/* SPDX-License-Identifier: MIT OR Apache-2.0
 *
 * Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>
 */
ENTRY(_start)
SECTIONS
{
    /* Set current address to the value from which the RPi starts execution */
    . = 0x10000;

    .text :
    {
        *(.text._start) *(.text*)
    }

    .rodata :
    {
        *(.rodata*)
    }

    .data :
    {
        *(.data*)
    }

    /* Section is zeroed in u64 chunks, align start and end to 8 bytes */
    .bss ALIGN(4):
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(4);
        __bss_end = .;
    }
. = ALIGN(4);
  . = . + 0x10000; /* 4kB of SVC stack space */
  stack_top = .;
  . = . + 0x100; /* some blank space */
  __heap_start = .;
    /DISCARD/ : { *(.comment*) }
}
