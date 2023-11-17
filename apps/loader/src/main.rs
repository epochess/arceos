#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#[cfg(feature = "axstd")]
use axstd::println;
const PLASH_START: usize = 0x22000000;
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_header_start = PLASH_START as *const u8;
    let apps_header_len = 8;
    let apps_start = (PLASH_START + apps_header_len) as *const u8;
    println!("Load payload ...");

    let len: &[u8] = unsafe { core::slice::from_raw_parts(apps_header_start, apps_header_len) };
    let app_size = bytes_to_usize(&len[..]);    
    println!("app len: {}", app_size);

    
    let code = unsafe { core::slice::from_raw_parts(apps_start, app_size) };
    println!("content: {:?}", code);
    
    // println!("content: {:?}", code);
    println!("Load payload ok!");
}

#[inline]
fn bytes_to_usize(bytes: &[u8]) -> usize {
    usize::from_be_bytes(bytes.try_into().unwrap())
}
