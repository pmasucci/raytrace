use rand::{thread_rng, Rng};
use std::ops::RangeInclusive;

pub fn random_f32() -> f32 {
    thread_rng().gen()
}

pub fn random_f32_range(range: RangeInclusive<f32>) -> f32 {
    thread_rng().gen_range(range)
}
