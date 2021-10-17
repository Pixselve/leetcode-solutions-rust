struct Solution {}

use std::cmp::max;
use std::collections::HashSet;

impl Solution {
    /// Given a string s, find the length of the longest substring without repeating characters.
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut count = 0;
        let mut characters: HashSet<char> = HashSet::new();
        let mut i = 0;
        let mut j = 0;
        while i < s.len() && j < s.len() {
            let char_at_j = s.chars().nth(j).unwrap();
            if characters.contains(&char_at_j) {
                characters.remove(&s.chars().nth(i).unwrap());
                i += 1;

            } else {
                j += 1;
                characters.insert(char_at_j);
                count = max(count, j - i);
            }
        }
        return count as i32;
    }
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::length_of_longest_substring(String::from("")), 0);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::length_of_longest_substring(String::from("a")), 1);
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::length_of_longest_substring(String::from("au")), 2);
    }

    #[test]
    fn test_7() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abc")),
            3
        );
    }

    #[test]
    fn test_8() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcd")),
            4
        );
    }

    #[test]
    fn test_9() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("dvdf")),
            3
        );
    }
}
