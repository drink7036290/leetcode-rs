pub struct Solution;

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

    fn search_peak(arr: &[i32], p: usize, v: usize, end: usize, rst: &mut usize) {
        // find first arr[p] > arr[p_end]
        let mut p_end = Self::binary_search(arr, arr[p], v, end);
        while (v < p_end) && (arr[p] == arr[p_end]) {
            p_end = Self::binary_search(arr, arr[p], v, p_end - 1);
        }

        if arr[p] <= arr[p_end] {
            p_end -= 1;
        }

        let tmp = Self::range(p + 1, p_end);
        if tmp < *rst {
            *rst = tmp;
        }
    }

    fn search_valley(arr: &[i32], v: usize, start: usize, p: usize, rst: &mut usize) {
        // find first arr[v_start] > arr[v]
        let mut v_start = Self::binary_search(arr, arr[v], start, p);
        while (v_start < p) && (arr[v_start] == arr[v]) {
            v_start = Self::binary_search(arr, arr[v], v_start + 1, p);
        }

        if arr[v_start] <= arr[v] {
            v_start += 1;
        }

        let tmp = Self::range(v_start, v - 1);
        if tmp < *rst {
            *rst = tmp;
        }
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

        // valley: last decrease
        let mut v = size - 1;

        // peak: first decrease
        let mut p = v; // init as last

        // from left to right
        for (i, val) in arr.iter().enumerate().skip(1) {
            if arr[i - 1] > *val {
                p = i - 1;
                break;
            }
        }

        // non-decreasing
        if p == size - 1 {
            return 0;
        }

        // from right to left
        for (rev_i, val) in arr.iter().rev().enumerate().skip(1) {
            let i = (size - 1) - rev_i; // ... 3 2 1 0
            if *val > arr[i + 1] {
                v = i + 1;
                break;
            }
        }

        if p == 0 && v == size - 1 {
            if arr[p] <= arr[v] {
                return size - 2;
            }

            return size - 1;
        }

        let mut rst = usize::MAX;

        // arr[p]
        Self::search_peak(arr, p, v, size - 1, &mut rst);

        // arr[v]
        Self::search_valley(arr, v, 0, p, &mut rst);

        // arr[p - 1]
        if 0 < p {
            p -= 1;
            Self::search_peak(arr, p, v, size - 1, &mut rst);
        }

        // arr[v + 1]
        if v < size - 1 {
            v += 1;
            Self::search_valley(arr, v, 0, p, &mut rst);
        }

        rst
    }

    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        Self::find_length_of_shortest_subarray_slice(&arr) as i32
    }
}
