struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = digits.clone();
        result[digits.len() - 1] += 1;
        for i in (1..result.len()).rev() {
            if result[i] == 10 {
                result[i - 1] += 1;
                result[i] = 0;
            }
        }

        if result.first().unwrap() == &10 {
            result[0] = 0;
            result.insert(0, 1);
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
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::plus_one(vec![1, 2, 9]), vec![1, 3, 0]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }
}
