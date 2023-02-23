struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut parentheses_stack: Vec<u8> = vec![];


        for character in s.chars() {
            match character {
                '{' => { parentheses_stack.push(0) }
                '(' => { parentheses_stack.push(1) }
                '[' => { parentheses_stack.push(2) }

                '}' => { if parentheses_stack.last().unwrap_or(&3).eq(&0) { parentheses_stack.pop(); } else { return false; } }
                ')' => { if parentheses_stack.last().unwrap_or(&3).eq(&1) { parentheses_stack.pop(); } else { return false; } }
                ']' => { if parentheses_stack.last().unwrap_or(&3).eq(&2) { parentheses_stack.pop(); } else { return false; } }
                _ => {}
            }
        }
        parentheses_stack.is_empty()
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn tests() {
        let input_and_solutions = vec![("()", true), ("()[]{}", true), ("(]", false), ("([)]", false), ("{[]}", true)];
        for input in input_and_solutions {
            assert_eq!(Solution::is_valid(input.0.parse().unwrap()), input.1, "we are testing addition with {}", input.0)
        }
    }
}