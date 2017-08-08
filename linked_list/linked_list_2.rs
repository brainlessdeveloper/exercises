struct Node<'a, T: 'a> {
    content: T,
    next: Option<&'a Node<'a, T>>,
}

impl<'a, T> Node<'a, T> {
    fn new(content: T) -> Self {
        Node {
            content,
            next: None,
        }
    }

    fn pluck_content(node: Self) -> T {
        node.content
    }

    fn next(&self) -> Option<&Node<T>> {
        self.next
    }
}

struct FLinkedList<'a, T: 'a> {
    head: Option<Node<'a, T>>,
    len: usize,
}

impl<'a, T> FLinkedList<'a, T>  {
    pub fn new() -> FLinkedList<'a, T> {
        FLinkedList {
            head: None,
            len: 0,
        }
    }

    pub fn prepend(&'a mut self, element: T) {
        let mut node = Node::new(element);
        let old_head = self.head.clone().as_ref();
        self.head = Some(node);
        node.next = old_head;
        self.len += 1;
    }
}

fn main() {
    println!("Singly linked list exercise.");
}

#[test]
fn prepend_extends_list_length() {
    let my_linked_list: FLinkedList<i32> = FLinkedList::new();
    my_linked_list.prepend(4);
    my_linked_list.prepend(2);
    assert_eq!(my_linked_list.len, 2);
}
