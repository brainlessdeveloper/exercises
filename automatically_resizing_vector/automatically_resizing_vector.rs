#![feature(alloc)]
#![feature(heap_api)]
#![feature(lang_items)]
#![feature(start)]
#![feature(unique)]
#![no_std]

extern crate alloc;

#[lang="panic_fmt"]
extern fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}

#[start]
fn start(argc: isize, argv: *const *const u8) -> isize {
    0
}

#[lang = "eh_personality"] extern fn eh_personality() {}

use alloc::heap;
use core::cmp;
use core::mem;
use core::ops::Deref;
use core::ptr::Unique;
use core::ptr::null_mut;
use core::ptr;
use core::slice;

struct Vec<T> {
    cap: usize,
    growth_factor: u8,
    len: usize,
    ptr: *mut T,
}

impl<T> Vec<T> {
    pub fn new() -> Vec<T> {
        Vec {
            cap: 0,
            growth_factor: 2,
            len: 0,
            ptr: null_mut(),
        }
    }

    pub fn with_capacity(cap: usize) {
        unimplemented!();
    }

    pub fn with_growth_factor(growth_factor: u8) {
        unimplemented!();
    }

    pub fn push(&mut self, item: T) {
        unsafe {
            if self.cap == self.len {
                let target_cap = self.cap * self.growth_factor as usize;
                self.resize(target_cap);
                self.push(item);
            } else {
                let end = self.ptr.offset(self.len as isize);
                unsafe ptr::write(end, item);
                self.len += 1;
            }
        }
    }

    fn resize(&mut self, target_cap: usize) -> Vec<T> {
        unsafe {
            self.cap = target_cap;
            let end = self.ptr.offset(self.len as isize);
        }
    }
}

fn main() {
    let super_vector: Vec<u8> = Vec::new();
}
