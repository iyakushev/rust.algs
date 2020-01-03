use std::io;
use std::io::Write;     // To use flush().

pub fn fib(position: u64) -> u64 {
    let mut prev = 0;
    let mut curr = 1;
    for _ in 2..position {
        let new = prev + curr;
        prev = curr;
        curr = new;
    }
    curr
}

pub fn fib_recursive(position: u64) -> u64 {
    if position == 0 || position == 1 {return 1}
    fib_recursive(position - 2) + fib_recursive(position -1)
}


fn main() -> io::Result<()> {
    print!("Please enter your value: ");
    io::stdout()
        .flush()?;   // Flush buffered output. 
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Could not read the string.");
    let parsed = value.trim().parse::<u64>().unwrap();
    println!("The result is {}", fib(parsed));
    Ok(())
}