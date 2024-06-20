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
    print_stack_trace();
}

fn print_stack_trace() {
    let mut fp: usize = 0;
    unsafe { asm!("mv {}, fp", out(reg) fp) };

    while fp != 0 {
        println!("fp = {:x}", fp);

        let fp_ptr: *const u64 = { unsafe { (fp as *const u64).offset(-2) } };
        fp = unsafe { (*fp_ptr).try_into().unwrap() };
    }
}
