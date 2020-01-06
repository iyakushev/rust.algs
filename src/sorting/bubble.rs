mod random;

fn bubble(v: &mut Vec<f64>) -> &mut Vec<f64> {
    for i in 0..v.len() {
        for j in 0..v.len() {
            if v[i] >= v[j] {
                v.swap(i, j);
            }
        }
    }
    v
}

fn main() {
    let mut v = random::generate_vec(10, -10., 22.);
    println!("Before: {:?}", v);
    bubble(&mut v);
    println!("After: {:?}", v);

}