pub struct Solution;

// Generated by ChatGPT o1-preview, which is both clean and concise, AWESOME!

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut left = 0;
        // Find the longest non-decreasing prefix
        while left + 1 < n && arr[left] <= arr[left + 1] {
            left += 1;
        }
        // If the whole array is non-decreasing
        if left == n - 1 {
            return 0;
        }
        let mut right = n - 1;
        // Find the longest non-decreasing suffix
        while right > 0 && arr[right - 1] <= arr[right] {
            right -= 1;
        }
        // Initialize result as minimum of removing prefix [0,right-1] or suffix [left+1,n-1]
        let mut result = (n - left - 1).min(right);
        let mut i = 0;
        let mut j = right;
        // Try to merge prefix and suffix
        while i <= left && j < n {
            if arr[i] <= arr[j] {
                // remove [i+1, j-1]
                result = result.min(j - i - 1);
                i += 1;
            } else {
                j += 1;
            }
        }
        result as i32
    }
}
