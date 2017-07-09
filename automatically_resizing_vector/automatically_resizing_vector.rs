#![feature(alloc)]
#![feature(heap_api)]

extern crate alloc;
extern crate core;

use alloc::heap;
use core::mem;
use core::ptr::null_mut;
use core::ptr;

#[derive(Debug)]
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
            let end = self.ptr.offset(self.len as isize) as *mut T;
            ptr::write(end, item);
            self.len += 1;
        }
    }

    pub fn at(&mut self, index: usize) -> T {
        unsafe {
            ptr::read::<T>(self.ptr.offset(index as isize))
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
        unimplemented!();
    }
}

fn main() {
    println!("Yo!");
}

#[test]
fn empty_vector_is_not_allocated() {
    let my_vector: FVec<&str> = FVec::new();
    assert_eq!(my_vector.ptr, null_mut());
}

#[test]
fn initial_push_sets_capacity_to_four() {
    let mut my_vector: FVec<&str> = FVec::new();
    my_vector.push("Hello");
    assert_eq!(my_vector.cap, 4);
}

#[test]
fn push_above_capacity_doubles_capacity() {
    let mut my_vector: FVec<&str> = FVec::new();
    my_vector.push("For");
    my_vector.push("you");
    my_vector.push("and");
    my_vector.push("for");
    my_vector.push("her");
    assert_eq!(my_vector.len, 5);
    assert_eq!(my_vector.cap, 8);
}

#[test]
fn access_by_index() {
    let mut my_vector: FVec<u8> = FVec::new();
    my_vector.push(20);
    my_vector.push(40);
    my_vector.push(10);
    assert_eq!(my_vector.at(0), 20);
    let last_index = my_vector.len - 1;
    assert_eq!(my_vector.at(last_index), 10);
}
