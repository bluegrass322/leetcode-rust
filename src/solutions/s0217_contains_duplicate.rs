use itertools::Itertools;

#[allow(dead_code)]
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    nums.into_iter().duplicates().collect::<Vec<_>>().len() > 0
}

#[cfg(test)]
mod s0217_tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(contains_duplicate(vec![1,2,3,1]), true);
    }

    #[test]
    fn case2() {
        assert_eq!(contains_duplicate(vec![1,2,3,4]), false);
    }

    #[test]
    fn case3() {
        assert_eq!(contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true);
    }
}

