use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut min_price = prices[0];
        let mut max_profit = 0;

        for price in prices {
            min_price = min(min_price, price);
            max_profit = max(max_profit, price - min_price);
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5)
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0)
    }
}
