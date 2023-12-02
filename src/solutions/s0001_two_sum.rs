use std::collections::HashSet;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut result: Vec<i32> = vec![];

    let mut checked: HashSet<usize> = HashSet::new();
    while i < nums.len() {
        let mut j = 0;
        while j < nums.len() {
            result = vec![i as i32, j as i32];

            if i != j && result.iter().sum::<i32>() == target {
                break;
            }
            j += 1;
        }
        i += 1;
    }

    result
}

#[cfg(test)]
mod s0001_tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9).sort(), vec![0, 1].sort());
    }

    #[test]
    fn case2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6).sort(), vec![1, 2].sort());
    }

    #[test]
    fn case3() {
        assert_eq!(two_sum(vec![3, 3], 6).sort(), vec![0, 1].sort());
    }
}
