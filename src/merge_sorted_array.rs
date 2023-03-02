impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut index_nums1 = m - 1;
        let mut index_nums2 = n - 1;
        let mut end_nums1 = m + n - 1;

        while index_nums1 >= 0 && index_nums2 >= 0 {
            if nums1[index_nums1 as usize] > nums2[index_nums2 as usize] {
                nums1[end_nums1 as usize] = nums1[index_nums1 as usize];
                index_nums1 -= 1;
            } else {
                nums1[end_nums1 as usize] = nums2[index_nums2 as usize];
                index_nums2 -= 1;
            }
            end_nums1 -= 1;
        }

        while index_nums2 >= 0 {
            nums1[end_nums1 as usize] = nums2[index_nums2 as usize];
            index_nums2 -= 1;
            end_nums1 -= 1;
        }
    }
}

struct Solution {}

