use core::alloc::Layout;
use core::ptr::NonNull;
use core::cmp::max;

use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};

pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    bstart: usize,
    bend: usize,
    buse: usize,
    pstart: usize,
    pend: usize,
    puse: usize,
    unused_size: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    /// Creates a new empty `EarlyAllocator`.
    pub const fn new() -> Self {
        Self {
            bstart: 0,
            pstart: 0,
            bend: 0,
            pend: 0,
            buse: 0,
            puse: 0,
            unused_size: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        let end = start + size;
        self.bstart = start;
        self.pstart = end;
        self.bend = start;
        self.pend = end;
        self.unused_size = size;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Err(AllocError::NoMemory)
    }
}

impl<const PAGE_SIZE: usize>  ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), core::mem::size_of::<usize>()),
        );

        let bend = self.bend;
        let pend = self.pend;

        if pend - bend >= size {
            let result = NonNull::new(bend as *mut u8);
            if let Some(result) = result {
                self.bend += size;
                self.buse += layout.size();
                return Ok(result)
            }
        }

        Err(AllocError::NoMemory)
    }

    fn dealloc(&mut self, _pos: NonNull<u8>, layout: Layout) {
        self.buse -= layout.size();
        if self.buse == 0 {
            self.bend = self.bstart;
        }
    }

    fn total_bytes(&self) -> usize {
        self.pstart - self.bstart
    }

    fn used_bytes(&self) -> usize {
        (self.bend - self.bstart) + (self.pstart - self.pend)
    }

    fn available_bytes(&self) -> usize {
        self.pend - self.bend
    }
}


impl<const PAGE_SIZE: usize>  PageAllocator for EarlyAllocator<PAGE_SIZE> {
    /// The size of a memory page.
    const PAGE_SIZE: usize = PAGE_SIZE;

    /// Allocate contiguous memory pages with given count and alignment.
    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        if align_pow2 % PAGE_SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }

        let align_pow2 = align_pow2 / PAGE_SIZE;
        if !align_pow2.is_power_of_two() {
            return Err(AllocError::InvalidParam);
        }

        let size = num_pages * PAGE_SIZE;
        if self.pend - self.bend >= size {
            self.pend -= size;
            self.puse += size;
            Ok(self.pend)
        } else {
            Err(AllocError::NoMemory)
        }
    }

    /// Deallocate contiguous memory pages with given position and count.
    fn dealloc_pages(&mut self, _pos: usize, num_pages: usize) {
        let size = num_pages * PAGE_SIZE;
        self.puse -= size;
        if self.puse == 0 {
            self.pend = self.pstart;
        }
    }

    /// Returns the total number of memory pages.
    fn total_pages(&self) -> usize {
        (self.pstart - self.bstart) / PAGE_SIZE
    }

    /// Returns the number of allocated memory pages.
    fn used_pages(&self) -> usize {
        self.puse / PAGE_SIZE
    }

    /// Returns the number of available memory pages.
    fn available_pages(&self) -> usize {
        (self.pend - self.bend) / PAGE_SIZE
    }
}