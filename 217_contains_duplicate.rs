#!/usr/bin/env rust-script
//! https://leetcode.com/problems/contains-duplicate/
//!
//! ```cargo
//! [dependencies]
//! itertools = "0.11.0"
//! ```

use itertools::Itertools;

fn main() {}

pub fn function(nums: Vec<i32>) -> bool {
    nums.into_iter().duplicates().collect::<Vec<_>>().len() > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(function(vec![1,2,3,1]), true);
    }

    #[test]
    fn case2() {
        assert_eq!(function(vec![1,2,3,4]), false);
    }

    #[test]
    fn case3() {
        assert_eq!(function(vec![1,1,1,3,3,4,3,2,4,2]), true);
    }
}

