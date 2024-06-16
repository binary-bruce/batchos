#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let answer = 42;
    println!(
        "Answer to the Ultimate Question of Life, the Universe, and Everything: {}",
        answer
    );
    0
}
