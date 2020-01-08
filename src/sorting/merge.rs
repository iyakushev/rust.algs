use std::marker::Copy;
mod random;

fn merge<T: Copy +  PartialOrd + Clone>(v: &mut [T], pivot: usize) {
    let mut ret = Vec::with_capacity(v.len());
    let mut left_it  = v[..pivot].iter().peekable();
    let mut right_it = v[pivot..].iter().peekable();

    loop {
        let value = match (left_it.peek(), right_it.peek()) {
            (None, None) => break,
            (Some(&val), None) => {left_it.next(); val.clone()},
            (Some(&left), Some(&right)) 
                if left <= right => {left_it.next(); left.clone()},
			(_, Some(&val)) => {right_it.next(); val.clone()}
        };
        ret.push(value);
    }
    v.copy_from_slice(ret.as_mut_slice());
}

fn merge_sort<T: Copy +  Clone + PartialOrd>(v: &mut [T]) {
    if v.len() <= 1 {return}

    let pivot = v.len()/2;
    merge_sort(&mut v[..pivot]);
    merge_sort(&mut v[pivot..]);
    merge(v, pivot);

}


fn main() {
    let mut v = random::generate_vec(10, -10, 30);
    println!("BEFORE: {:?}", v);
    merge_sort(v.as_mut_slice());
    println!("AFTER: {:?}", v);
}