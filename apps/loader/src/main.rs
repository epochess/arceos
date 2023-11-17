#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#[cfg(feature = "axstd")]
use axstd::println;
const PLASH_START: usize = 0x22000000;

#[repr(C)]
struct Header<const APP_NUM: usize> {
    app0_start: usize,
    app_len: [usize; APP_NUM],
}

impl<const APP_NUM: usize> Header<APP_NUM> {
    fn new(start: usize) -> Self {
        Header {
            app0_start: start + core::mem::size_of::<usize>() * (APP_NUM),
            app_len: [0usize; APP_NUM],
        }
    }

    fn get_app_lens(start: usize) -> Self {
        let mut header = Self::new(start);
        let mut base = 0;
        const LEN: usize = core::mem::size_of::<usize>();
        for i in 0..APP_NUM {
            let bytes = unsafe { core::slice::from_raw_parts((start + base) as *const u8 , LEN) };
            header.app_len[i] = usize::from_be_bytes(bytes.try_into().unwrap());
            base += LEN;
        }
        header
    }
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let header = Header::<2>::get_app_lens(PLASH_START);

    println!("Load payload ...");

    let mut base = 0;
    for i in 0..2 {
        println!("load app {i}:");
        println!("app len: {}", header.app_len[i]);
        let apps_start = (header.app0_start + base) as *const u8;
        // println!("{:#x}", header.app0_start + base);
        let code = unsafe { core::slice::from_raw_parts(apps_start, header.app_len[i]) };
        println!("content: {:?}", code);
        base += header.app_len[i];
    }

    // println!("content: {:?}", code);
    println!("Load payload ok!");
}

// #[inline]
// fn bytes_to_usize(bytes: &[u8]) -> usize {
//     usize::from_be_bytes(bytes.try_into().unwrap())
// }
