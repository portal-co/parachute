#![no_std]
#![cfg_attr(feature = "allocator-api", feature(allocator_api))]

use core::alloc::GlobalAlloc;

use linked_list_allocator::Heap;
use spin::Mutex;
#[cfg(feature = "alloc")]
extern crate alloc;

pub struct BootMem(Mutex<BootMemInternal>);
enum BootMemInternal {
    #[cfg(feature = "limine")]
    DormantLimine {
        memmap: &'static limine::request::MemoryMapRequest,
    },
    #[cfg(feature = "limine")]
    Limine {
        memmap: &'static limine::response::MemoryMapResponse,
        heap: Heap,
    },
}
impl BootMem {
    pub unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut lock = self.0.lock();
        loop {
            match &mut *lock {
                #[cfg(feature = "limine")]
                BootMemInternal::DormantLimine { memmap } => {
                    let Some(memmap) = memmap.get_response() else {
                        return core::ptr::null_mut();
                    };
                    let mut largest: Option<(u64, u64)> = None;
                    for e in memmap.entries().iter() {
                        use limine::memory_map::EntryType;

                        if e.entry_type == EntryType::USABLE {
                            let size = e.length;
                            if largest.is_none() || size > largest.unwrap().1 {
                                largest = Some((e.base, size));
                            }
                        }
                    }
                    *lock = BootMemInternal::Limine {
                        memmap,
                        heap: unsafe {
                            let Some((base, size)) = largest else {
                                use core::ptr::null_mut;

                                return null_mut();
                            };
                            Heap::new(base as *mut u8, size as usize)
                        },
                    };
                }
                #[cfg(feature = "limine")]
                BootMemInternal::Limine { memmap, heap } => {
                    return heap
                        .allocate_first_fit(layout)
                        .ok()
                        .map_or(core::ptr::null_mut(), |allocation| allocation.as_ptr());
                }
            }
        }
    }

    pub unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let mut lock = self.0.lock();
        loop {
            match &mut *lock {
                #[cfg(feature = "limine")]
                BootMemInternal::DormantLimine { memmap } => {
                    let Some(memmap) = memmap.get_response() else {
                        return;
                    };
                    let mut largest: Option<(u64, u64)> = None;
                    for e in memmap.entries().iter() {
                        use limine::memory_map::EntryType;

                        if e.entry_type == EntryType::USABLE {
                            let size = e.length;
                            if largest.is_none() || size > largest.unwrap().1 {
                                largest = Some((e.base, size));
                            }
                        }
                    }
                    *lock = BootMemInternal::Limine {
                        memmap,
                        heap: unsafe {
                            let Some((base, size)) = largest else {
                                return;
                            };
                            Heap::new(base as *mut u8, size as usize)
                        },
                    };
                }
                #[cfg(feature = "limine")]
                BootMemInternal::Limine { memmap, heap } => {
                    use core::ptr::NonNull;

                    return match NonNull::new(ptr) {
                        Some(non_null_ptr) => {
                            unsafe { heap.deallocate(non_null_ptr, layout) };
                        }
                        None => {}
                    };
                }
            }
        }
    }
}
unsafe impl GlobalAlloc for BootMem {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe { self.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        unsafe { self.dealloc(ptr, layout) }
    }
}
