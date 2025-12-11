/*
File: app_no_std/src/main.rs

Для бинарника no_std нужно: 
* runtime:
* no_std в корне
* panic handler
* аллокатор, если используешь Vec, Box, String
* Таргет, поддерживающий no_std (например thumbv6m-none-eabi)

*/
#![no_std]
#![no_main]
#![allow(static_mut_refs)]

extern crate lib_ffi;
extern crate alloc;

use core::panic::PanicInfo;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr;

/// Простая кучи 4KB
#[link_section = ".uninit"]
static mut HEAP_MEMORY: [u8; 4096] = [0; 4096];

struct SimpleAllocator {
    heap_start: UnsafeCell<usize>,
    heap_end: UnsafeCell<usize>,
    next: UnsafeCell<usize>,
}

// Говорим компилятору, что статический аллокатор безопасно использовать
unsafe impl Sync for SimpleAllocator {}

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let current = *self.next.get();
        let alloc_start = (current + layout.align() - 1) & !(layout.align() - 1);
        let alloc_end = alloc_start + layout.size();

        if alloc_end > *self.heap_end.get() {
            ptr::null_mut()
        } else {
            // Используем ptr::write, чтобы не было warnings
            ptr::write(self.next.get(), alloc_end);
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: SimpleAllocator = SimpleAllocator {
    heap_start: UnsafeCell::new(0),
    heap_end: UnsafeCell::new(0),
    next: UnsafeCell::new(0),
};

// Panic handler для no_std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        ptr::write(ALLOCATOR.heap_start.get(), HEAP_MEMORY.as_ptr() as usize);
        ptr::write(
            ALLOCATOR.heap_end.get(),
            HEAP_MEMORY.as_ptr() as usize + HEAP_MEMORY.len(),
        );
        ptr::write(ALLOCATOR.next.get(), HEAP_MEMORY.as_ptr() as usize);
    }

    let sum = lib_ffi::try_sum_vec();

    match sum {
        Ok(v) => {
            let _ = v; // используем значение
        }
        Err(_) => loop {}, // OOM
    }

    loop {}
}
