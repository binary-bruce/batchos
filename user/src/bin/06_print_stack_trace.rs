#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use core::arch::asm;

#[no_mangle]
fn main() -> i32 {
    unsafe {
        println!("");
        println!("main");
        print_stack_trace()
    };

    a();
    0
}

#[no_mangle]
fn a() {
    unsafe {
        println!("");
        println!("a");
        print_stack_trace()
    };

    b();
}

#[no_mangle]
fn b() {
    unsafe {
        println!("");
        println!("b");
        print_stack_trace()
    };

    c();
}

#[no_mangle]
fn c() {
    unsafe {
        println!("");
        println!("c");
        print_stack_trace()
    };

    d();
}

#[no_mangle]
fn d() {
    unsafe {
        println!("");
        println!("d");
        print_stack_trace()
    };

    e();
}

#[no_mangle]
fn e() {
    unsafe {
        println!("");
        println!("e");
        print_stack_trace()
    };
}

#[inline(always)]
unsafe fn print_stack_trace() {
    let mut fp: *const usize;
    unsafe { asm!("mv {}, fp", out(reg) fp) };

    println!("== Begin stack trace ==");
    while fp != core::ptr::null() {
        let saved_ra = *fp.sub(1);
        let saved_fp = *fp.sub(2);
        println!("ra = 0x{:08x}, fp = 0x{:08x}", saved_ra, saved_fp);

        fp = saved_fp as *const usize
    }
    println!("== End stack trace ==");
}
