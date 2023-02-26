use std::cmp::max;
use std::iter;

struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_iter = a
            .chars()
            .map(|x| x.to_digit(2).unwrap())
            .rev()
            .chain(iter::repeat(0).take(max(0, b.len() - a.len())));
        let b_iter = b
            .chars()
            .map(|x| x.to_digit(2).unwrap())
            .rev()
            .chain(iter::repeat(0).take(max(0, a.len() - b.len())));

        let (result, carry) = a_iter
            .zip(b_iter)
            .fold(("".to_owned(), 0), |(result, carry), (a_value, b_value)| {
                let sum = a_value + b_value + carry;
                match sum {
                    1 => ("1".to_owned() + &result, 0),
                    2 => ("0".to_owned() + &result, 1),
                    3 => ("1".to_owned() + &result, 1),
                    _ => ("0".to_owned() + &result, 0),
                }
            });
        if carry == 1 {
            return "1".to_owned() + &result
        }
        result
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::add_binary("1".to_owned(), "1".to_owned()), "10".to_owned());
    }
}
