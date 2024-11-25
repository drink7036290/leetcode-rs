pub struct Solution;

impl Solution {
    fn trap_slice(height: &[i32]) -> i32 {
        // water += min(max_l, max_r) - curr

        let size = height.len();
        if size <= 2 {
            return 0;
        }

        let mut water = 0;

        let mut l = 0;
        let mut r = size - 1;
        let mut max_l = height[l];
        let mut max_r = height[r];

        l += 1;
        r -= 1;

        let mut curr;

        while l <= r {
            if max_l <= max_r {
                curr = height[l];
                if max_l <= curr {
                    max_l = curr;
                } else {
                    water += max_l - curr;
                }

                l += 1;
            } else {
                curr = height[r];
                if curr >= max_r {
                    max_r = curr;
                } else {
                    water += max_r - curr;
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
