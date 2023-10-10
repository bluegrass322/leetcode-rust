#!/usr/bin/env rust-script
//! https://leetcode.com/problems/valid-anagram/
//!
//! ```cargo
//! [dependencies]
//! itertools = "0.11.0"
//! ```

use itertools::Itertools;

fn main() {}

pub fn is_anagram(s: String, t: String) -> bool {
    let mut result = false;
    let t_vec = t.chars().collect::<Vec<char>>();

    for pattern in s.chars().permutations(s.len()) {
        if pattern == t_vec {
            result = true;
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(is_anagram("anagram".to_string(), "nagaram".to_string()), true)
    }

    #[test]
    fn case2() {
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false)
    }
}

