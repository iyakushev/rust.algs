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
    let fib = Fib::new();
    let values: Vec<u64> = fib.take(20).collect();
    println!("Fibonacci iterator -> {:?}", values);
}