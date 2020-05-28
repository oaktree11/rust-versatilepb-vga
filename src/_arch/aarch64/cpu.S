// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>

.section ".text._start"

.global _start

_start:
/* set SVC stack */
 	LDR sp, =svc_stack

	/* copy vector table to address 0 */

	// try to copy vector table to 0 while MMU is off
        ldr r3, =vectors_start
	ldr r4, =vectors_end
	mov r0, #0
	ldr r1, =vectors_start
again:	
	ldr r2, [r1], #4
	str r2, [r0], #4
        cmp r1, r4
	bne again

//  BL copy_vectors
  /* get Program Status Register */
  MRS r0, cpsr
  /* go in IRQ mode */
  BIC r1, r0, #0x1F
  ORR r1, r1, #0x12
  MSR cpsr, r1
  /* set IRQ stack */
  LDR sp, =irq_stack
  /* Enable IRQs */
  BIC r0, r0, #0x80
  /* go back in Supervisor mode */
  MSR cpsr, r0
  /* jump to main */
  BL runtime_init
  B .

.align 4
irq_handler:

  sub	lr, lr, #4
  stmfd	sp!, {r0-r10, fp, ip, lr}

  bl	IRQ_handler  

  ldmfd	sp!, {r0-r10, fp, ip, pc}^

lock:
	mrs r0, cpsr
	orr r0, r0, #0x80
	msr cpsr, r0
	mov pc, lr
unlock:
	mrs r0, cpsr
	BIC r0, r0, #0x80
	msr cpsr, r0
	mov pc, lr
	
// int_on()/int_off(): turn on/off IRQ interrupts
int_on: // may pass parameter in r0
  MRS r0, cpsr
  MSR cpsr, r0
  mov pc, lr	

int_off: // may pass parameter in r0
  MRS r1, cpsr
  MOV r0, r1
  ORR r1, r1, #0x80   // set bit means MASK off IRQ interrupt 
  MSR cpsr, r1
  mov pc, lr	

vectors_start:

  LDR PC, reset_handler_addr
/*  LDR PC, undef_handler_addr
  LDR PC, swi_handler_addr
  LDR PC, prefetch_abort_handler_addr
  LDR PC, data_abort_handler_addr
  B .
  LDR PC, irq_handler_addr
  LDR PC, fiq_handler_addr*/

reset_handler_addr:          .word reset_handler
/*undef_handler_addr:          .word undef_handler
swi_handler_addr:            .word swi_handler
prefetch_abort_handler_addr: .word prefetch_abort_handler
data_abort_handler_addr:     .word data_abort_handler
irq_handler_addr:            .word irq_handler
fiq_handler_addr:            .word fiq_handler*/

vectors_end:



// old code

 //   mrs     x1, mpidr_el1   // Read Multiprocessor Affinity Register
  //  and     x1, x1, #3      // Clear all bits except [1:0], which hold core id
  //  cbz     x1, 2f          // Jump to label 2 if we are core 0
//1:  wfe                     // Wait for event
  //  b       1b              // In case an event happened, jump back to 1
//2:                          // If we are here, we are core0
 //   ldr     x1, =_start     // Load address of function "_start()"
 //   mov     sp, x1          // Set start of stack to before our code, aka first
                            // address before "_start()"
 //   bl      runtime_init    // Jump to the "runtime_init()" kernel function
 //   b       1b              // We should never reach here. But just in case,
                            // park this core aswell
