struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = String::new();

        let none = String::new();
        let model = strs.get(0).unwrap_or(&none);
        for (i, current_character) in model.chars().enumerate() {
            for x in &strs {
                if match x.chars().nth(i) {
                    None => true,
                    Some(char) => char != current_character,
                } {
                    return result;
                }
            }
            result.push(current_character);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::longest_common_prefix(vec![]), "");
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight"),
            ]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car"),
            ]),
            ""
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("a"),
                String::from("a"),
                String::from("a"),
            ]),
            "a"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("ab"),
                String::from("ab"),
                String::from("ab"),
            ]),
            "ab"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("ab"),
                String::from("a"),
                String::from("ab"),
            ]),
            "a"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("ab"),
                String::from("ac"),
                String::from("ab"),
            ]),
            "a"
        );
    }
}
