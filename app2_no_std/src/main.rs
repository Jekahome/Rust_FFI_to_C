/*
File: app2_no_std/src/main.rs

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
extern crate linked_list_allocator;

use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[link_section = ".uninit"]
static mut HEAP_MEMORY: [u8; 4096] = [0; 4096];

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Инициализация linked_list_allocator
        ALLOCATOR.lock().init(HEAP_MEMORY.as_mut_ptr(), HEAP_MEMORY.len());
    }

    let sum = lib_ffi::try_sum_vec();

    match sum {
        Ok(v) => { let _ = v; },
        Err(_) => loop {}, // OOM
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
