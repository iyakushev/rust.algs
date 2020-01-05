use std::io;
use std::io::Write;

pub struct List {
    head: Option<Box<Node>>,
    len: u64
}

struct Node {
    value: i64,
    next: Option<Box<Node>> // Box is a dynamic heap allocator.
}

impl Node {
    fn new(val: i64) -> Node {
        Node{value: val, next: None}
    }

    fn link(&mut self, node: Option<Box<Node>>) {
        self.next = node;
    }
}

impl List {
    pub fn new(val: i64) -> List {
        List {
            head: Some(Box::new(Node::new(val))),
            len: 1
       }
    }

    pub fn push(&mut self, val: i64) {
        let mut node = Node::new(val);
        node.link(self.head.take());
        self.head = Some(Box::new(node));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<i64> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn ls(&self) {
        let mut node = &self.head;
        print!("CONTENTS: ");
        loop {
            match node {
                Some(ptr) => {
                    print!("{}->", ptr.value);
                    node = &ptr.next;
                }
                None => {println!("Null"); break}
            }
        }
        io::stdout()
            .flush()  // Flush buffered output.
            .unwrap();
    }
}

fn main() {
    let mut list = List::new(61);
    list.push(62);
    list.push(63);
    list.push(64);
    list.push(42);
    let popped = list.pop();
    list.ls();
    println!("{} was popped from the head", popped.unwrap());
}