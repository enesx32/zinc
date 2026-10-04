#![no_std]

pub mod alloc;

pub use alloc::BumpAllocator;

/// The starting address of Zinc's heap.
///
/// This address is currently used as the beginning of the memory region
/// available to Zinc's allocator.
pub const HEAP_START: usize = 0x200000;

/// The size of Zinc's heap in bytes.
///
/// Zinc currently reserves 1 MiB of memory for dynamic allocations.
pub const HEAP_SIZE: usize = 1024 * 1024;

/// The global allocator used by Zinc OS.
#[global_allocator]
pub static ALLOCATOR: BumpAllocator = BumpAllocator::new();