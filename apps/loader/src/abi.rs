#![allow(unused)]

pub const SYS_HELLO: usize = 1;
pub const SYS_PUTCHAR: usize = 2;

pub static mut ABI_TABLE: [usize; 16] = [0; 16];

pub fn register_abi(num: usize, handle: usize) {
    unsafe { ABI_TABLE[num] = handle; }
}

pub fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
}

pub fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
}