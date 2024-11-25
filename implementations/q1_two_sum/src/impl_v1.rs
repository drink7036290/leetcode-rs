pub struct Solution;

use std::collections::HashMap;

impl Solution {
    fn two_sum_slice(nums: &[i32], target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        let mut result = vec![0; 2];

        for (index, value) in nums.iter().enumerate() {
            match map.get(&(target - value)) {
                Some(pair_index) => {
                    result[0] = *pair_index as i32;
                    result[1] = index as i32;
                    break;
                }

                None => map.insert(*value, index),
            };
        }

        result
    }
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::two_sum_slice(&nums, target)
    }
}
