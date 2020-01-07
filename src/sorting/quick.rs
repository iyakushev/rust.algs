mod random;

fn partition<T: PartialOrd>(v: &mut [T]) -> usize {
    let pivot = v.len() / 2;
    let last_index = v.len() - 1;
 
    v.swap(pivot, last_index);  // Move pivot to the end of the vec
 
    let mut border = 0;
    for i in 0..last_index {
        if v[i] <= v[last_index] {
            v.swap(i, border);  // Move everything below pivot to the left
            border += 1;       // Update the border value for the left part
        }
    }
 
    v.swap(border, last_index); // Swap border value with pivot
    border
}


fn quick_sort<T: PartialOrd>(v: &mut [T]) {
    if v.len() <= 1 {return}
    
    let pivot = partition(v);

    quick_sort(&mut v[..pivot]);
    quick_sort(&mut v[pivot..]);
}


fn main() {
    let mut v = random::generate_vec(10, -10, 30);
    println!("BEFORE: {:?}", v);
    quick_sort(v.as_mut_slice());
    println!("AFTER: {:?}", v);
}