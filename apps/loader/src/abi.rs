pub const SYS_HELLO: usize = 1;
pub const SYS_PUTCHAR: usize = 2;
pub const SYS_TERMINATE: usize = 3;

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

pub fn abi_terminate() -> ! {
    arceos_api::sys::ax_terminate()
}

// pub fn abi_call(abi_num: usize, arg: usize) {
//     match abi_num {
//         SYS_HELLO       => abi_hello(),
//         SYS_PUTCHAR     => abi_putchar(arg as u8 as char),
//         SYS_TERMINATE   => abi_terminate(),
//         _               => panic!("unknow abi call"),
//     }
// }