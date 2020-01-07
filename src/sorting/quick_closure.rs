mod random;

//              A way to define a template type
//              VVVVV
fn partition<T,F:Fn(&T,&T)->bool> (v: &mut [T], cmp: &F) -> usize {
    let pivot = v.len() / 2;
    let last_index = v.len() - 1;
 
    v.swap(pivot, last_index);  // Move pivot to the end of the vec
 
    let mut border = 0;
    for i in 0..last_index {
        if cmp(&v[i], &v[last_index]) {
            v.swap(i, border);  // Move everything below pivot to the left
            border += 1;       // Update the border value for the left part
        }
    }
 
    v.swap(border, last_index); // Swap border value with pivot
    border
}


fn quick_sort<T,F>(v: &mut [T], f: &F) 
    where F: Fn(&T,&T)->bool { // Template type also can be stated this way
    if v.len() <= 1 {return}
    
    let pivot = partition(v, f);

    quick_sort(&mut v[..pivot], f);
    quick_sort(&mut v[pivot..], f);
}


fn main() {
    let mut v = random::generate_vec(10, -10, 30);
    println!("BEFORE: {:?}", v);
    
    quick_sort(v.as_mut_slice(), &|x,y| x >= y);
    println!("AFTER: {:?}", v);

    quick_sort(v.as_mut_slice(), &|x,y| x <= y);
    println!("CLOSURE CHANGE: {:?}", v);
}