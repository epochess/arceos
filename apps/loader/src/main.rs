#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

mod abi;
mod parse;

use parse::Header;
use abi::*;

const PLASH_START: usize = 0x22000000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let header = Header::<2, PLASH_START>::get_app_lens();

    println!("Load payload ...");

    for i in 0..2 {
        let apps_start = header.app_start(i) as *const u8;
        let app_size = header.app_len(i);
        
        println!("load app {i}:");
        println!("app len: {}", app_size);
        println!("{:#x}", header.app_start(i));
        
        let code = unsafe { core::slice::from_raw_parts(apps_start, app_size) };
        println!("content: {:?}, address: [{:?}]", code, code.as_ptr());
        
        // app running aspace
        // SBI(0x80000000) -> App <- Kernel(0x80200000)
        // 0xffff_ffc0_0000_0000

        const RUN_START: usize = 0xffff_ffc0_8010_0000;
        let run_code = unsafe {
            core::slice::from_raw_parts_mut(RUN_START as *mut u8, app_size)
        };
        run_code.copy_from_slice(code);

        println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());

        println!("Execute app ...");
        register_abi(SYS_HELLO, abi_hello as usize);
        register_abi(SYS_PUTCHAR, abi_putchar as usize);
        register_abi(SYS_TERMINATE, abi_terminate as usize);

        println!("Execute app ...");
        let arg0: u8 = b'A';
    
        // execute app
        unsafe { core::arch::asm!("
            li      t0, {abi_num}
            slli    t0, t0, 3
            la      t1, {abi_table}
            add     t1, t1, t0
            ld      t1, (t1)
            jalr    t1
            li      t2, {run_start}
            jalr    t2
            j       .",
            run_start = const RUN_START,
            abi_table = sym ABI_TABLE,
            abi_num = const SYS_TERMINATE,
            // abi_num = const SYS_HELLO,
            // abi_num = const SYS_PUTCHAR,
            in("a0") arg0,
        )}
    }

    // println!("content: {:?}", code);
    println!("Load payload ok!");
}

// #[inline]
// fn bytes_to_usize(bytes: &[u8]) -> usize {
//     usize::from_be_bytes(bytes.try_into().unwrap())
// }
