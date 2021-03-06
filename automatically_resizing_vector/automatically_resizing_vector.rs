#![feature(alloc)]
#![feature(allocator_api)]

extern crate alloc;
extern crate core;

use alloc::heap::{Alloc, Layout, Heap};
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
            self.prepare_for_len_increase();
            let end = self.ptr.offset(self.len as isize) as *mut T;
            ptr::write(end, item);
            self.len += 1;
        }
    }

    pub fn at(&mut self, index: usize) -> T {
        unsafe {
            ptr::read(self.ptr.offset(index as isize))
        }
    }

    pub fn insert(&mut self, item: T, index: usize) {
        unsafe {
            self.prepare_for_len_increase();
            let destination = self.ptr.offset((index + 1) as isize) as *mut T;
            let count = self.len - index;
            ptr::copy(self.ptr.offset(index as isize) as *const T, destination, count);
            ptr::write(self.ptr.offset(index as isize), item);
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> T {
        unsafe {
            self.len -= 1;
            ptr::read(self.ptr.offset(self.len as isize))
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
            let item_align = mem::align_of::<T>();
            let target_ptr = Heap::default().realloc(
                self.ptr as *mut u8,
                Layout::from_size_align(item_size * self.cap, item_align).expect("Failed to construct previous layout during reallocation"),
                Layout::from_size_align(item_size * target_cap, item_align).expect("Failed to construct new layout during reallocation"),
            ).expect("Failed to reallocate.");
            self.cap = target_cap;
            self.ptr = target_ptr as *mut _;
        }
    }

    fn allocate(&mut self, target_cap: usize) {
        unsafe {
            let item_size = mem::size_of::<T>();
            let target_ptr = Heap::default().alloc(
                Layout::from_size_align(item_size * target_cap, mem::align_of::<T>()).expect("Failed to construct new layout during initial allocation"),
            ).expect("Failed during initial allocation");
            self.cap = target_cap;
            self.ptr = target_ptr as *mut _;
        }
    }

    fn prepare_for_len_increase(&mut self) {
        let growth_factor = self.growth_factor;
        let current_cap = self.cap;
        if self.cap == self.len {
            if self.cap == 0 {
                self.allocate(4);
            } else {
                self.resize(current_cap * growth_factor as usize);
            }
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
fn push_above_capacity_multiplies_capacity_by_growth_factor() {
    let mut my_vector: FVec<&str> = FVec::new();
    my_vector.push("For");
    my_vector.push("you");
    my_vector.push("and");
    my_vector.push("for");
    my_vector.push("her");
    assert_eq!(my_vector.len, 5);
    assert_eq!(my_vector.cap, 8); // Default growth value is 2
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

#[test]
fn items_still_there_after_reallocation() {
    let mut my_vector: FVec<&str> = FVec::new();
    let initial_cap = my_vector.cap;
    my_vector.push("For");
    my_vector.push("you");
    my_vector.push("and");
    my_vector.push("for");
    my_vector.push("her");
    assert!(my_vector.cap != initial_cap);
    assert_eq!(my_vector.at(0), "For");
    assert_eq!(my_vector.at(1), "you");
    assert_eq!(my_vector.at(2), "and");
    assert_eq!(my_vector.at(3), "for");
    assert_eq!(my_vector.at(4), "her");
}

#[test]
fn insert() {
    let mut my_vector: FVec<&str> = FVec::new();
    my_vector.push("For");
    my_vector.push("me");
    my_vector.push("only");
    my_vector.insert("and", 2);
    assert_eq!(my_vector.at(3), "only");
    my_vector.insert("you", 3);
    assert_eq!(my_vector.at(0), "For");
    assert_eq!(my_vector.at(1), "me");
    assert_eq!(my_vector.at(2), "and");
    assert_eq!(my_vector.at(3), "you");
    assert_eq!(my_vector.at(4), "only");
    assert_eq!(my_vector.len, 5);
}

#[test]
fn insert_when_empty() {
    let mut my_vector: FVec<&str> = FVec::new();
    my_vector.insert("Tators", 0);
    assert_eq!(my_vector.at(0), "Tators");
    assert_eq!(my_vector.len, 1);
}

#[test]
fn insert_at_zero() {
    let mut my_vector: FVec<&str> = FVec::new();
    my_vector.push("me");
    my_vector.push("only");
    my_vector.insert("For", 0);
    assert_eq!(my_vector.at(0), "For");
    assert_eq!(my_vector.at(1), "me");
    assert_eq!(my_vector.at(2), "only");
    assert_eq!(my_vector.len, 3);
}

#[test]
fn insert_at_end() {
    let mut my_vector: FVec<&str> = FVec::new();
    my_vector.push("For");
    my_vector.push("me");
    let last_index = my_vector.len;
    my_vector.insert("only", last_index);
    assert_eq!(my_vector.at(0), "For");
    assert_eq!(my_vector.at(1), "me");
    assert_eq!(my_vector.at(2), "only");
    assert_eq!(my_vector.len, 3);
}

#[test]
fn pop() {
    let mut my_vector: FVec<&str> = FVec::new();
    my_vector.push("Something");
    my_vector.push("bad");
    assert_eq!(my_vector.pop(), "bad");
    my_vector.push("good");
    assert_eq!(my_vector.at(1), "good");
}
