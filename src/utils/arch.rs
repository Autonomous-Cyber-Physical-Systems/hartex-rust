//! # Machine specific
//!
//! Defines functions which are defined majorly in assembly. Thus, might change for one board to another.

/// Returns the MSB of `val`. It is written using CLZ instruction.
pub fn get_msb(val: u32) -> usize {
    let mut res;
    unsafe {
        asm!("clz $1, $0"
        : "=r"(res)
        : "0"(val)
        );
    }
    res = 32 - res;
    if res > 0 {
        res -= 1;
    }
    return res;
}

/// Returns true if Currently the Kernel is operating in Privileged mode.
pub fn is_privileged() -> bool {
    let val: u32;
    unsafe {
        asm!("mrs $0, CONTROL"
            : "=r"(val)
            :
        )
    };
    !((val & 1) == 1)
}

/// Creates an SVC Interrupt
pub fn svc_call() {
    unsafe {
        asm!("svc 1");
    }
}

/// PendSV interrupt handler does the actual context switch in the Kernel.
// pub fn pendSV_handler() {
//     unsafe {
//         asm!(
//             "
//             /* Disable interrupts: */
// 	cpsid	i

// 	/*
// 	Exception frame saved by the NVIC hardware onto stack:
// 	+------+
// 	|      | <- SP before interrupt (orig. SP)
// 	| xPSR |
// 	|  PC  |
// 	|  LR  |
// 	|  R12 |
// 	|  R3  |
// 	|  R2  |
// 	|  R1  |
// 	|  R0  | <- SP after entering interrupt (orig. SP + 32 bytes)
// 	+------+

// 	Registers saved by the software (PendSV):
// 	+------+
// 	|  R7  |
// 	|  R6  |
// 	|  R5  |
// 	|  R4  |
// 	|  R11 |
// 	|  R10 |
// 	|  R9  |
// 	|  R8  | <- Saved SP (orig. SP + 64 bytes)
// 	+------+
// 	*/

// 	/* Save registers R4-R11 (32 bytes) onto current PSP (process stack
// 	   pointer) and make the PSP point to the last stacked register (R8):
// 	   - The MRS/MSR instruction is for loading/saving a special registers.
// 	   - The STMIA inscruction can only save low registers (R0-R7), it is
// 	     therefore necesary to copy registers R8-R11 into R4-R7 and call
// 	     STMIA twice. */
// 	mrs	r0, psp
// 	subs	r0, #16
// 	stmia	r0!,{r4-r7}
// 	mov	r4, r8
// 	mov	r5, r9
// 	mov	r6, r10
// 	mov	r7, r11
// 	subs	r0, #32
// 	stmia	r0!,{r4-r7}
// 	subs	r0, #16

// 	/* Save current task's SP: */
// 	ldr	r2, =os_curr_task
// 	ldr	r1, [r2]
// 	str	r0, [r1]

// 	/* Load next task's SP: */
// 	ldr	r2, =os_next_task
// 	ldr	r1, [r2]
// 	ldr	r0, [r1]

// 	/* Load registers R4-R11 (32 bytes) from the new PSP and make the PSP
// 	   point to the end of the exception stack frame. The NVIC hardware
// 	   will restore remaining registers after returning from exception): */
// 	ldmia	r0!,{r4-r7}
// 	mov	r8, r4
// 	mov	r9, r5
// 	mov	r10, r6
// 	mov	r11, r7
// 	ldmia	r0!,{r4-r7}
// 	msr	psp, r0

// 	/* EXC_RETURN - Thread mode with PSP: */
// 	ldr r0, =0xFFFFFFFD

// 	/* Enable interrupts: */
// 	cpsie	i

// 	bx	r0
//             "
//         )
//     };
// }

use crate::kernel::task_management::{all_tasks,os_curr_task,os_next_task};

pub fn pendSV_handler() {
	let handler = unsafe { &mut all_tasks };
    let task_curr = &handler.task_control_blocks[1];
    if handler.started {
        unsafe {
            let ctask = task_curr.as_ref().unwrap();
            os_curr_task = ctask;
            // ctask.save_context();
        }
    } else {
        handler.started = true;
    }
    handler.curr_tid = 2;
    let task_next = &handler.task_control_blocks[2];
    let ctask = task_next.as_ref().unwrap();
    ctask.load_context();
    // unsafe {
    //     os_next_task = ctask;
    //     cortex_m::peripheral::SCB::set_pendsv();
    // }
}