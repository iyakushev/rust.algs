use std::io;
use std::io::Write;

pub fn gcd(a: u64, b: u64) -> u64 {
    let mut t_a = a; // function variables to escape mutation of the args
    let mut t_b = b;
    while t_b > 0 {
        t_a %= t_b;  // Assign remainder into "t_a"
        
       std::mem::swap(&mut t_a, &mut t_b); // Swap values
    }
    t_a // return "t_a"
}

pub fn gcd_recursive(a: u64, b: u64) -> u64 {
    if a == 0 {return b}
    if b == 0 {return a}
    if a >= b {return gcd_recursive(a % b, b)}
    else {return gcd_recursive(a, b % a)}
}


fn main(){
    print!("Please enter your value: ");
    io::stdout()
        .flush()  // Flush buffered output.
        .unwrap(); 
    let mut values = String::new();
    io::stdin().read_line(&mut values).expect("Could not read the string.");
    
    let args: Vec<u64> = values
                        .split_whitespace()         // Split the args by the whitespace
                        .map(|x| x.parse::<u64>().expect("Parse error."))  // Dereference each value
                        .collect();                 // Collect them into a Vector: u64

    println!("GCD of {} and {} is {}", args[0], args[1], gcd(args[0], args[1]));
}
