use once_cell::sync::Lazy;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::ops::RangeInclusive;

pub const SEED: u64 = 12345; // Use a fixed seed for reproducibility
pub const NUM_PROPTEST_CASES: u32 = 10; // Number of proptest cases to generate

// Key: 0 <= key <= 10^5
pub fn key_range() -> RangeInclusive<i32> {
    0..=100_000
}

// Value: 0 <= value <= 10^9
pub fn value_range() -> RangeInclusive<i32> {
    0..=1_000_000_000
}

// Capacity: 1 <= capacity <= 10^4
pub fn capacity_range() -> RangeInclusive<usize> {
    1..=10_000
}

pub static CAPACITY: Lazy<usize> =
    Lazy::new(|| StdRng::seed_from_u64(SEED).gen_range(capacity_range()));
