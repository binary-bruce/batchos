//! Trap handling functionality
//!
//! For rCore, we have a single trap entry point, namely `__alltraps`. At
//! initialization in [`init()`], we set the `stvec` CSR to point to it.
//!
//! All traps go through `__alltraps`, which is defined in `trap.S`. The
//! assembly language code does just enough work restore the kernel space
//! context, ensuring that Rust code safely runs, and transfers control to
//! [`trap_handler()`].
//!
//! It then calls different functionality based on what exactly the exception
//! was. For example, timer interrupts trigger task preemption, and syscalls go
//! to [`syscall()`].

use core::arch::global_asm;

use context::TrapContext;
use riscv::register::{
    scause,
    scause::{Exception, Trap},
    stval, stvec,
};

use crate::syscall::syscall;

/// Trap Context
pub mod context;

global_asm!(include_str!("trap/trap.S"));

/// init CSR `stvec` as the entry of `__alltraps`
pub fn init() {
    extern "C" {
        fn __alltraps();
    }

    unsafe { stvec::write(__alltraps as usize, stvec::TrapMode::Direct) };
}

#[no_mangle]
/// entry point to handle interrupt, exception or system call from user space
pub(crate) fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read(); // trap cause
    let stval = stval::read(); // extra value of the trap

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4; // move forward to next instruction
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            ();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            ();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }

    cx
}
