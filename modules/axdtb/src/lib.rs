#![cfg_attr(all(not(test), not(doc)), no_std)]
#![feature(doc_auto_cfg)]

extern crate alloc;
use core::str;
// use alloc::string::String;
use alloc::vec::Vec;
use fdt;
// use axdtb::util::SliceRead;
pub struct DtbInfo {
    pub memory_addr: usize,
    pub memory_size: usize,
    pub mmio_regions: Vec<(usize, usize)>,
}

pub fn parse_dtb(dtb_pa: usize) -> Result<DtbInfo, &'static str> {
    let fdt = unsafe { fdt::Fdt::from_ptr(dtb_pa as *const u8) }.unwrap();
    let mut regions = Vec::new();

    for node in fdt.find_all_nodes("/soc/virtio_mmio") {
        let virtio_mmio_range = node.property("reg").unwrap().value;
        let io_start = u64::from_be_bytes(virtio_mmio_range[0..8].try_into().unwrap()) as usize;
        let io_size = u64::from_be_bytes(virtio_mmio_range[8..16].try_into().unwrap()) as usize;
        regions.push((io_start, io_size));
    }

    let memory_addr = fdt.memory().regions().next().unwrap().starting_address as usize;
    let memory_size = fdt.memory().regions().next().unwrap().size.unwrap() as usize;

    Ok(DtbInfo { memory_addr, memory_size, mmio_regions: regions })
}
