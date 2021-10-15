use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut element_to_index = HashMap::new();

    for i in 0..nums.len() {
        element_to_index.insert(nums[i], i);
    }
    for i in 0..nums.len() {
        match element_to_index.get(&(target - nums[i])) {
            None => {}
            Some(value) => {
                if value != &i {
                    return vec![i as i32, *value as i32];
                }
            }
        }
    }
    return vec![];
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_4() {
        assert_eq!(two_sum(vec![-1, -2, -3, -4, -5], -8), vec![2, 4]);
    }
    #[test]
    fn test_5() {
        assert_eq!(two_sum(vec![2,1,9,4,4,56,90,3], 8), vec![3, 4]);
    }
}
