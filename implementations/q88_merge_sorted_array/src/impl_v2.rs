pub struct Solution;

impl Solution {
    // https://rust-lang.github.io/rust-clippy/master/index.html#ptr_arg
    // pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32)

    pub fn merge(nums1: &mut [i32], m: i32, nums2: &[i32], n: i32) {
        let mut merge_index = (m + n - 1) as usize;

        let mut is_nums1_done = true;
        let mut index1 = 0;
        if m > 0 {
            index1 = (m - 1) as usize;
            is_nums1_done = false;
        }

        for v in nums2.iter().rev() {
            if !is_nums1_done {
                while nums1[index1] > *v {
                    nums1[merge_index] = nums1[index1];
                    merge_index -= 1;

                    match index1.checked_sub(1) {
                        Some(result) => index1 = result,
                        None => {
                            is_nums1_done = true;
                            break;
                        }
                    }
                }
            }

            nums1[merge_index] = *v;
            match merge_index.checked_sub(1) {
                Some(result) => merge_index = result,
                None => break,
            }
        }
    }
}
