use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let s_counts = count_chars(&s);
    let t_counts = count_chars(&t);

    s_counts == t_counts
}

fn count_chars(string: &String) -> HashMap<char, i32> {
    let mut hash = HashMap::<char, i32>::new();

    for s in string.chars() {
        let count = hash.entry(s).or_insert(0);
        *count += 1;
    }

    hash
}

/*
use itertools::Itertools;

#[allow(dead_code)]
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
*/

#[cfg(test)]
mod s0242_tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        )
    }

    #[test]
    fn case2() {
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false)
    }
}
