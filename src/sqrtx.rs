struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let max_iter = 50;
        let change = 0.5;
        let mut sqrt = x as f64;
        for _ in 0..max_iter {
            let result = (sqrt + x as f64 / sqrt) / 2f64;
            if (sqrt - result).abs() < change {
                return result.floor() as i32;
            }
            sqrt = result;
        }

        sqrt.floor() as i32
    }
}