pub struct Solution;

impl Solution {
    fn get_maximum_xor_slice(nums: &[i32], maximum_bit: i32) -> Vec<i32> {
        let size = nums.len();
        let max_value = (1 << maximum_bit) - 1;

        let mut rst = vec![0; size];

        let mut xor_total = 0;
        let mut i = 0;
        while i < size {
            xor_total ^= nums[i];
            rst[size - 1 - i] = max_value ^ xor_total;

            i += 1;
        }

        rst
    }
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        Self::get_maximum_xor_slice(&nums, maximum_bit)
    }
}
