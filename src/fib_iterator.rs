struct Fib {
    prev: u64,
    curr: u64
}

impl Iterator for Fib {
    type Item = u64; // A value that should be returned

    fn next(&mut self) -> Option<Self::Item> {
        let new = self.prev + self.curr;
        self.prev = self.curr;
        self.curr = new;
        Some(new)
    }
}

impl Fib {
    fn new() -> Fib {
        Fib{
            prev: 0,
            curr: 1
        }
    }
}

fn main() {
    let fib = Fib::new();  // Initialize the structure
    let values: Vec<u64> = fib.take(20).collect();  // Take the first 20 values and store them in a vector
    println!("Fibonacci iterator -> {:?}", values); // Prints a vector in debug mode
                                                    // Otherwise it would requre a custom fmt implementation for Vec
}
