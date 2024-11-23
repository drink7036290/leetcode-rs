pub mod impl_v1;

#[cfg(test)]
use impl_v1::Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(
            6,
            Solution::number_of_subarrays([1, 1, 2, 1, 1].to_vec(), 1)
        );
        assert_eq!(
            5,
            Solution::number_of_subarrays([1, 1, 2, 1, 1].to_vec(), 2)
        );
        assert_eq!(
            24,
            Solution::number_of_subarrays([2, 2, 2, 1, 2, 2, 1, 2, 2, 2].to_vec(), 1)
        );
        assert_eq!(
            0,
            Solution::number_of_subarrays([2, 2, 2, 1, 2, 2, 1, 2, 2, 2].to_vec(), 3)
        );
        assert_eq!(
            16,
            Solution::number_of_subarrays([2, 2, 2, 1, 2, 2, 1, 2, 2, 2].to_vec(), 2)
        );
        assert_eq!(0, Solution::number_of_subarrays([2, 4, 6].to_vec(), 1));
        assert_eq!(
            2,
            Solution::number_of_subarrays([1, 1, 2, 1, 1].to_vec(), 3)
        );
    }
}
