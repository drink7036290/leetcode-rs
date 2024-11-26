pub struct Solution;

impl Solution {
    // https://rust-lang.github.io/rust-clippy/master/index.html#ptr_arg
    // pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32)

    pub fn merge(nums1: &mut [i32], m: i32, nums2: &[i32], n: i32) {
        assert_eq!(nums2.len(), n as usize);
        if n == 0 {
            //println!("{:?}", nums1);
            return;
        }

        if m == 0 {
            nums1
                .iter_mut()
                .zip(nums2.iter())
                .for_each(|(v1, v2)| *v1 = *v2);

            //println!("{:?}", nums1);
            return;
        }

        let mut tail1 = m as usize - 1;
        let mut i = (m + n) as usize - 1;
        //dbg!((m, n));

        /* nums1.iter_mut().rev().for_each(|v| {

            *v = 0;
        }); */

        let mut is_nums1_done = false;

        for tail2 in (0..n as usize).rev() {
            //dbg!((i, tail1, tail2));
            //dbg!((&nums1, &nums2));
            while nums1[tail1] > nums2[tail2] {
                nums1[i] = nums1[tail1];
                i -= 1;

                if let Some(v) = tail1.checked_sub(1) {
                    tail1 = v;
                } else {
                    is_nums1_done = true;
                    break;
                }

                //dbg!((i, tail1, tail2));
                //dbg!((&nums1, &nums2));
            }

            nums1[i] = nums2[tail2];

            if is_nums1_done {
                nums1
                    .iter_mut()
                    .zip(nums2.iter())
                    .take(tail2)
                    .for_each(|(v1, v2)| *v1 = *v2);

                break;
            }

            i -= 1;
        }
    }
}
