use std::ops::AddAssign;
mod random;

fn count_sort<T>(v: &mut [T], max: usize) 
    where T: Into<usize> + From<u8> + Copy + AddAssign{
    let mut count = vec!(0; max + 1);  // Vec of zeros with len = v.len
    
    for &num in v.iter() {
        count[num.into() as usize] += 1;
    }

    println!("COUNT ARR: {:?}", count);

    let mut i = 0;
    let mut val = T::from(0);  // Type-safe cast to <T> from u8
    for &idx in count.iter() {
        for _ in 0..idx {
            v[i] = val;
            i += 1;
        }
        val += T::from(1);
    }
}

// Won't work with floats and negatives. 
// It can, but this will only
// bloat the algorithm.
fn main() {
    let (min, max) = (0, 30);
    let mut v = random::generate_vec(10, min, max);
    println!("BEFORE: {:?}", v);
    count_sort(v.as_mut_slice(), max);
    println!("AFTER: {:?}", v);
}