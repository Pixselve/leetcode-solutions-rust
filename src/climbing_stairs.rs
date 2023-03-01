struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 1;
        let mut b = 2;
        for _ in 1..n {
            let sum = a + b;
            a = b;
            b = sum;
        }
        a
    }
}

