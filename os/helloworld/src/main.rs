#![no_std]
#![no_main]

#[macro_use]
extern crate axlog;
extern crate axruntime;

#[no_mangle]
fn main() {
    println!("Hello, world!");
}