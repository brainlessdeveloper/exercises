#![feature(alloc)]
#![feature(heap_api)]

extern crate alloc;
extern crate core;

use alloc::heap;
use core::mem;
use core::ptr::null_mut;
use core::ptr;

struct FVec<T> {
    cap: usize,
    growth_factor: u8,
    len: usize,
    ptr: *mut T,
}

impl<T> FVec<T> {
    pub fn new() -> FVec<T> {
        FVec {
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
            let growth_factor = self.growth_factor;
            let current_cap = self.cap;
            if self.cap == self.len {
                if self.cap == 0 {
                    self.allocate(4);
                } else {
                    self.resize(current_cap * growth_factor as usize);
                }
            }
            let end = self.ptr.offset(self.len as isize);
            ptr::write(end, item);
            self.len += 1;
        }
    }

    fn resize(&mut self, target_cap: usize) {
        if target_cap > self.cap {
            self.reallocate(target_cap);
        } else {
            self.truncate(target_cap);
        }
    }

    fn reallocate(&mut self, target_cap: usize) {
        unsafe {
            let item_size = mem::size_of::<T>();
            let target_ptr = heap::reallocate(
                self.ptr as *mut _,
                item_size * self.cap,
                item_size * target_cap,
                mem::align_of::<T>(),
            );
            self.cap = target_cap;
            self.ptr = target_ptr as *mut _;
        }
    }

    fn allocate(&mut self, target_cap: usize) {
        unsafe {
            let item_size = mem::size_of::<T>();
            let target_ptr = heap::allocate(item_size * target_cap, mem::align_of::<T>());
            self.cap = target_cap;
            self.ptr = target_ptr as *mut _;
        }
    }

    fn truncate(&mut self, target_cap: usize) {
        println!("Calling truncate");
        unimplemented!();
    }
}

fn main() {
    let mut super_vector: FVec<&str> = FVec::new();
    super_vector.push("Hello");
}
