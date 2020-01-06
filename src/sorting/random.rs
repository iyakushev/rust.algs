extern crate rand;
use rand::Rng;

pub fn generate_vec(size: i32, range_start: f64, range_end: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let v: Vec<f64> = (0..size).map(|_| rng.gen_range(range_start, range_end)).collect();
    v
}
