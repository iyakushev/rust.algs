mod random;

fn insertion<T: PartialOrd>(v: &mut [T]) {
    for i in 1..v.len() {
        for j in (1..i+1).rev() {
            if v[j-1] <= v[j] {break}
            v.swap(j-1, j);
        }
    }
}


fn main() {
    let mut v = random::generate_vec(10, -10., 30.);
    println!("BEFORE: {:?}", v);
    insertion(v.as_mut_slice());
    println!("AFTER: {:?}", v);
}