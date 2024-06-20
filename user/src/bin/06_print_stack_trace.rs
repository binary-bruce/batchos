#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use core::arch::asm;

#[no_mangle]
fn main() -> i32 {
    foo();
    0
}

fn foo() {
    let _tmp = 0;
    bar();
}

fn bar() {
    let _tmp1 = 1;
    let _tmp2 = 2;
    print_sum(1, 2);
}

fn print_sum(left: i32, right: i32) {
    let sum = left + right;
    print(sum);
}

fn print(_value: i32) {
    unsafe { print_stack_trace() };
}

unsafe fn print_stack_trace() {
    let mut fp: *const usize;
    unsafe { asm!("mv {}, fp", out(reg) fp) };

    println!("== Begin stack trace ==");
    while fp != core::ptr::null() {
        let saved_ra = *fp.sub(1);
        let saved_fp = *fp.sub(2);
        println!("ra = 0x{:016x}, fp = 0x{:016x}", saved_ra, saved_fp);

        fp = saved_fp as *const usize
    }
    println!("== End stack trace ==");
}
