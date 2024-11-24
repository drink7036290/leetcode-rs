pub struct Solution;

use std::cmp::min;

impl Solution {
    fn range(start: usize, end: usize) -> usize {
        if start > end {
            return usize::MAX;
        }

        end - start + 1
    }

    fn binary_search(arr: &[i32], target: i32, mut start: usize, mut end: usize) -> usize {
        let mut mid = start;

        while start <= end {
            // start + (end - start) / 2
            match start.checked_add((end - start) >> 1) {
                Some(result) => mid = result,
                None => return start,
            }

            if arr[mid] == target {
                return mid;
            }

            if arr[mid] < target {
                match mid.checked_add(1) {
                    Some(result) => start = result,
                    None => break,
                }
            } else {
                match mid.checked_sub(1) {
                    Some(result) => end = result,
                    None => break,
                }
            }
        }

        mid
    }

    fn find_length_of_shortest_subarray_slice(arr: &[i32]) -> usize {
        let size = arr.len();
        match size {
            0 | 1 => {
                return 0;
            }
            2 => {
                if arr[0] <= arr[1] {
                    return 0;
                }
                return 1;
            }
            _ => {}
        }

        let mut right = 0;

        // from right to left
        for (rev_i, val) in arr.iter().rev().enumerate().skip(1) {
            let right_next = size - rev_i; // (size - 1) - rev_i + 1 // ... 3 2 1 0
            if *val > arr[right_next] {
                right = right_next;
                break;
            }
        }

        let end = size - 1;
        let mut rst = right; // [0, right-1] : zero left portion, right portion only

        // from left to right, add left portion
        for (left, val) in arr.iter().enumerate() {
            if left >= right {
                break;
            }
            if (left > 0) && (arr[left - 1] > *val) {
                break;
            }

            // right > end : zero right portion
            if right <= end {
                // find first r in [right, end] such that arr[left] > arr[r]
                let mut r = Self::binary_search(arr, *val, right, end);
                while (right < r) && (*val == arr[r]) {
                    r = Self::binary_search(arr, *val, right, r - 1);
                }

                right = r;
                if *val > arr[r] {
                    right += 1;
                }
            }

            rst = min(rst, Self::range(left + 1, right - 1));
        }

        rst
    }

    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        //Self::find_length_of_shortest_subarray_slice(&arr) as i32

        Self::find_length_of_shortest_subarray_slice(&arr) as i32
    }
}
