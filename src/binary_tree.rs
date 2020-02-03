enum BTree<T> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

struct Node<T> {
    value: T,
    left: BTree<T>,
    right: BTree<T>
}

impl<T: Ord> BTree<T> {
    pub fn add(&mut self, value: T) {
        match *self {
            BTree::Empty => 
                *self = BTree::NonEmpty(
                            Box::new(
                                Node {
                                    value: value,
                                    left: BTree::Empty,
                                    right: BTree::Empty,        
                                })),
            BTree::NonEmpty(ref mut node) =>
                if value <= node.value {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
        }
    }

    pub fn contains(&self, value: T) -> bool {
        match &self {
            BTree::NonEmpty(ref node) => 
                if value < node.value {
                    node.left.contains(value)
                } else if value > node.value {
                    node.right.contains(value)
                } else if value == node.value {
                    true
                } else {false},
            BTree::Empty => false
        }
    }

    pub fn new(value: T) -> Self {
        BTree::NonEmpty(Box::new(Node {
            value: value,
            left: BTree::Empty,
            right: BTree::Empty
        }))
    }
}

fn main() {
    let mut tree = BTree::new("Jupyter");
    tree.add("Mercury");
    tree.add("Pluto");
    tree.add("Earth");
    tree.add("Sun");

    println!(">> Does Pluto enter the tree? {}", tree.contains("Pluto"));
    println!(">> Does Mars enter the tree? {}", tree.contains("Mars"));
}