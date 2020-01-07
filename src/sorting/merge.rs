mod random;

fn merge() {
    
}


fn main() {
    let mut v = random::generate_vec(10, -10., 30.);
    println!("BEFORE: {:?}", v);
    merge(v.as_mut_slice());
    println!("AFTER: {:?}", v);
}