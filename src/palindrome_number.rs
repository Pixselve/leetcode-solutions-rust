struct Solution{}


impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // if x < 0 {return false}
        let mut result = 0;
        let mut current = x;
        while current > 0 {
            let current_digit = current % 10;
            current /= 10;
            result *= 10;
            result += current_digit;
        }
        result == x
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn tests() {
        let input_and_solutions = vec![(121, true), (-121, false), (10, false), (-101, false), (123321, true)];
        for input in input_and_solutions {
            assert_eq!(Solution::is_palindrome(input.0), input.1)
        }
    }
}
