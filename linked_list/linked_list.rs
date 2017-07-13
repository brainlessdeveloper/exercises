#![feature(alloc)]
#![feature(shared)]

extern crate alloc;
extern crate core;

use alloc::boxed::{Box};
use core::ptr::{Shared};

struct Node<T> {
    content: T,
    next: Option<Shared<Node<T>>>,
}

impl<T> Node<T> {
    fn new(content: T) -> Self {
        Node {
            next: None,
            content,
        }
    }

    fn pluck_content(node: Box<Self>) -> T {
        node.content
    }
}

struct FLinkedList<T> {
    head: Option<Shared<Node<T>>>,
    len: usize,
}

impl<T> FLinkedList<T> {
    pub fn new() -> FLinkedList<T> {
        FLinkedList {
            head: None,
            len: 0,
        }
    }

    pub fn prepend(&mut self, element: T) {
        let node = Box::new(Node::new(element));
        self.prepend_node(node);
    }

    pub fn pop_head(&mut self) -> Option<T> {
        self.pop_head_node().map(Node::pluck_content)
    }

    pub fn at(&self, index: usize) -> Option<T> {
        match self.node_at(index) {
            Some(node) => Node::pluck_content(node),
            _ => None,
        }
    }

    fn prepend_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = self.head;
            self.head = Some(Shared::new(Box::into_raw(node)));
            self.len += 1;
        }
    }

    fn pop_head_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            self.len -= 1;
            node
        })
    }

    fn node_at(&self, index: usize) -> Option<Box<Node<T>>> {
        let node = self.head;
        for i in [0..index] {
            node = node.next;
        }
        node
    }
}

fn main() {
    println!("Singly linked list exercise.");
}

#[test]
fn prepend_extends_list_length() {
    let mut my_linked_list: FLinkedList<i32> = FLinkedList::new();
    my_linked_list.prepend(4);
    my_linked_list.prepend(2);
    assert_eq!(my_linked_list.len, 2);
}

#[test]
fn prepend_and_pop_head_work() {
    let mut my_linked_list: FLinkedList<&str> = FLinkedList::new();
    my_linked_list.prepend("there");
    my_linked_list.prepend("hello");
    assert_eq!(my_linked_list.pop_head(), Some("hello"));
    assert_eq!(my_linked_list.pop_head(), Some("there"));
    assert_eq!(my_linked_list.pop_head(), None);
}

#[test]
fn node_at_works() {
    let mut my_linked_list: FLinkedList<&str> = FLinkedList::new();
    my_linked_list.prepend("Hello");
    my_linked_list.prepend("World");
    assert_eq!(my_linked_list.at(0), Some("Hello"));
    assert_eq!(my_linked_list.at(1), Some("World"));
}
