#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)] 

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

mod abi;
use abi::*;

const PLASH_START: usize = 0x22000000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_start = PLASH_START as *const u8;
    // Dangerous!!! We need to get accurate size of apps.
    let apps_size = 20; 
    println!("Load payload ...");

    // let code = unsafe { core::slice::from_raw_parts(apps_start, apps_size) };
    // println!("content: {:#x}", bytes_to_usize(&code[..8]));

    let load_code = unsafe { core::slice::from_raw_parts(apps_start, apps_size) };
    println!(
        "load code {:?}; address [{:?}]",
        load_code,
        load_code.as_ptr()
    );

    // app running aspace
    // SBI(0x80000000) -> App <- Kernel(0x80200000)
    // 0xffff_ffc0_0000_0000

    const RUN_START: usize = 0xffff_ffc0_8010_0000;
    
    let run_code = unsafe { 
        core::slice::from_raw_parts_mut(RUN_START as *mut u8, apps_size) 
    };
    run_code.copy_from_slice(load_code);
    
    println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());

    println!("Execute app ...");
    // // execute app
    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);

    // execute app
    unsafe { core::arch::asm!("
        la      a7, {abi_table}
        li      t2, {run_start}
        jalr    t2",
        run_start = const RUN_START,
        abi_table = sym ABI_TABLE,
    )}
}

// #[inline]
// fn bytes_to_usize(bytes: &[u8]) -> usize {
//     usize::from_be_bytes(bytes.try_into().unwrap())
// }
