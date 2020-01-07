extern crate rand;
use rand::Rng;

pub fn generate_vec<T: rand::distributions::uniform::SampleUniform + Copy>(size: i32, range_start: T, range_end: T) -> Vec<T> {
    let mut rng = rand::thread_rng();
    let v: Vec<T> = (0..size).map(|_| rng.gen_range(range_start, range_end)).collect();
    v
}
