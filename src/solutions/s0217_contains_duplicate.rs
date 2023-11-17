use std::collections::HashSet;
use std::iter::FromIterator;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let hash_set: HashSet<&i32> = HashSet::from_iter(nums.iter());
    nums.len() != hash_set.len()
}

/* Time Limit Exceeded
 * pub fn contains_duplicate(nums: Vec<i32>) -> bool {
 *     let mut result = false;
 *
 *     for (i, n) in nums.iter().enumerate() {
 *         for (j, m) in nums.iter().enumerate() {
 *             if i != j && n == m {
 *                 result = true;
 *                 break;
 *             }
 *         }
 *     }
 *     result
 * }
*/

/*
 * use itertools::Itertools;
 *
 * #[allow(dead_code)]
 * pub fn contains_duplicate(nums: Vec<i32>) -> bool {
 *     // my code
 *     // nums.into_iter().duplicates().collect::<Vec<_>>().len() > 0
 *
 *    // fixed by clippy
 *     !nums.into_iter().duplicates().collect::<Vec<_>>().is_empty()
 * }
 */

#[cfg(test)]
mod s0217_tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    }

    #[test]
    fn case2() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    }

    #[test]
    fn case3() {
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }
}
