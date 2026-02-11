#![no_std]
#![cfg_attr(feature = "allocator-api", feature(allocator_api))]

use spin::Mutex;
#[cfg(feature = "alloc")]
extern crate alloc;

pub struct BootMem(Mutex<BootMemInternal>);
enum BootMemInternal{
    #[cfg(feature = "limine")]
    DormantLimine{
        memmap: &'static limine::request::MemoryMapRequest,
    },
    
}