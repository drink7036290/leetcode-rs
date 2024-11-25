pub struct Solution;

impl Solution {
    fn trap_slice(height: &[i32]) -> i32 {
        // water += min(max_l, max_r) - curr

        let mut water = 0;

        let mut l = 0;
        let mut r = height.len() - 1;
        let mut max_l = 0;
        let mut max_r = 0;

        while l <= r {
            if height[l] <= height[r] {
                if max_l <= height[l] {
                    max_l = height[l];
                } else {
                    water += max_l - height[l];
                }

                l += 1;
            } else {
                if height[r] >= max_r {
                    max_r = height[r];
                } else {
                    water += max_r - height[r];
                }

                r -= 1;
            }
        }

        water
    }

    pub fn trap(height: Vec<i32>) -> i32 {
        Self::trap_slice(&height)
    }
}
