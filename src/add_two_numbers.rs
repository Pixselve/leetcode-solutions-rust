// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn append(&mut self, val: i32) {
        match self.next {
            None => self.next = Some(Box::from(ListNode::new(val))),
            Some(ref mut value) => value.append(val),
        }
    }
    fn from_vec(vec: Vec<i32>) -> Self {
        let mut result = ListNode::new(vec[0]);
        for i in 1..vec.len() {
            result.append(vec[i])
        }
        result
    }
}

pub struct Solution {}

impl ListNode {
    fn create_option_box(val: i32, next: Option<Box<Self>>) -> Option<Box<Self>> {
        Some(Box::from(ListNode { val, next }))
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let one = ListNode::create_option_box(1, None);
        match (l1, l2) {
            (Some(l1_value), Some(l2_value)) => {
                let calc_result = l1_value.val + l2_value.val;
                if calc_result >= 10 {
                    ListNode::create_option_box(
                        calc_result - 10,
                        Self::add_two_numbers(
                            Self::add_two_numbers(l1_value.next, one),
                            l2_value.next,
                        ),
                    )
                } else {
                    ListNode::create_option_box(
                        calc_result,
                        Self::add_two_numbers(l1_value.next, l2_value.next),
                    )
                }
            }
            (Some(l1_value), None) | (None, Some(l1_value)) => {
                if l1_value.val >= 10 {
                    ListNode::create_option_box(
                        l1_value.val - 10,
                        Self::add_two_numbers(one, l1_value.next),
                    )
                } else {
                    Some(l1_value)
                }
            }
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        let a = Some(Box::from(ListNode::from_vec(vec![2, 4, 3])));
        let b = Some(Box::from(ListNode::from_vec(vec![5, 6, 4])));
        let c = Some(Box::from(ListNode::from_vec(vec![7, 0, 8])));

        assert_eq!(Solution::add_two_numbers(a, b), c);
    }

    #[test]
    fn test_2() {
        let a = Some(Box::from(ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9])));
        let b = Some(Box::from(ListNode::from_vec(vec![9, 9, 9, 9])));
        let c = Some(Box::from(ListNode::from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1])));

        assert_eq!(Solution::add_two_numbers(a, b), c);
    }
}
