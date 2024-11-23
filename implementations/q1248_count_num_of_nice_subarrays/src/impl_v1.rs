pub struct Solution;

impl Solution {
    fn is_even(num: i32) -> bool {
        (num & 1) == 0
    }
    fn number_of_subarrays_slice(nums: &[i32], k: i32) -> usize {
        let size = nums.len();
        let mut rst = 0;
        let mut cn_odd = 0;

        let mut lb = 0;
        let mut l = 0;
        let mut r = l;
        let mut rb = r;

        while rb < size {
            // update L
            while l < size && Self::is_even(nums[l]) {
                l += 1;
            }
            if l == size {
                return rst;
            }
            cn_odd += 1;

            // update R
            if k == 1 {
                r = l;
            } else if rb > 0 {
                r = rb;
            } else {
                // rb == 0
                r = l + 1;

                while cn_odd < k {
                    while r < size && Self::is_even(nums[r]) {
                        r += 1;
                    }
                    if r == size {
                        return rst;
                    }
                    cn_odd += 1;

                    r += 1;
                }
                r -= 1; // restore the last one
            }

            // update RB
            rb = r + 1;
            while rb < size && Self::is_even(nums[rb]) {
                rb += 1;
            }

            // update rst
            let mut l_com = 1;
            if l != lb {
                l_com = l - lb;

                if Self::is_even(nums[lb]) {
                    l_com += 1;
                }
            }

            let r_com = rb - r; // even rb == size
            rst += l_com * r_com;

            // update LB
            lb = l;
            l += 1;
            cn_odd -= 1;
        }

        rst
    }
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        Self::number_of_subarrays_slice(&nums, k) as i32
    }
}
