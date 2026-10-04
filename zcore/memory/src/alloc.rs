use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

/// A simple bump allocator.
///
/// `BumpAllocator` allocates memory by moving a pointer forward through
/// a fixed region of memory.
///
/// Allocated memory is not currently reused after being freed.
///
/// This allocator is simple and useful for the early stages of Zinc OS.
pub struct BumpAllocator {
    /// The starting address of the heap.
    heap_start: AtomicUsize,

    /// The address immediately after the end of the heap.
    heap_end: AtomicUsize,

    /// The address where the next allocation will begin.
    next: AtomicUsize,
}

impl BumpAllocator {
    /// Creates an uninitialised `BumpAllocator`.
    ///
    /// The allocator does not have a usable heap until `init()` is called.
    pub const fn new() -> Self {
        Self {
            heap_start: AtomicUsize::new(0),
            heap_end: AtomicUsize::new(0),
            next: AtomicUsize::new(0),
        }
    }

    /// Initialises the allocator with a heap region.
    ///
    /// `heap_start` specifies the starting address of the heap.
    ///
    /// `heap_size` specifies the size of the heap in bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the specified memory region is valid,
    /// available for use, and does not overlap with other important memory.
    pub unsafe fn init(&self, heap_start: usize, heap_size: usize) {
        self.heap_start.store(heap_start, Ordering::Relaxed);
        self.heap_end.store(heap_start + heap_size, Ordering::Relaxed);
        self.next.store(heap_start, Ordering::Relaxed);
    }
}

/// Implements Rust's global allocation interface.
///
/// This allows Rust types such as `Vec`, `Box`, and `String` to request
/// dynamically allocated memory from the Zinc heap.
unsafe impl GlobalAlloc for BumpAllocator {
    /// Allocates a block of memory from the heap.
    ///
    /// `layout` contains the requested size and alignment.
    ///
    /// Returns a pointer to the allocated memory.
    ///
    /// Returns a null pointer if there is not enough space remaining
    /// in the heap.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let heap_end = self.heap_end.load(Ordering::Relaxed);
        let mut current = self.next.load(Ordering::Relaxed);

        loop {
            let aligned = (current + layout.align() - 1) & !(layout.align() - 1);

            let end = match aligned.checked_add(layout.size()) {
                Some(end) => end,
                None => return null_mut(),
            };

            if end > heap_end {
                return null_mut();
            }

            match self.next.compare_exchange_weak(
                current,
                end,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return aligned as *mut u8,
                Err(actual) => current = actual,
            }
        }
    }

    /// Releases an allocation.
    ///
    /// The bump allocator does not currently reuse freed memory,
    /// so this function does nothing.
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}