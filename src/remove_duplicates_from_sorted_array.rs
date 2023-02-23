pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() { return 0; }
        let mut current_digit = nums[0];
        let mut next_digit_index: i32 = 1;
        for i in 1..nums.len() {
            if current_digit == nums[i] {} else {
                nums[next_digit_index as usize] = nums[i];
                next_digit_index += 1;
                current_digit = nums[i];
            }
        }
        next_digit_index
    }
}