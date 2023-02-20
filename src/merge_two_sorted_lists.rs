struct Solution {}

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
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(list1_unwraped), Some(list2_unwraped)) => {
                let mut next;
                if list1_unwraped.val < list2_unwraped.val {
                    next = Box::from(ListNode::new(list1_unwraped.val));
                    next.next =
                        Solution::merge_two_lists(list1_unwraped.next, Some(list2_unwraped));
                } else {
                    next = Box::from(ListNode::new(list2_unwraped.val));
                    next.next =
                        Solution::merge_two_lists(Some(list1_unwraped), list2_unwraped.next);
                };
                Some(next)
            }
            (Some(list1_unwraped), None) => {
                let mut next = Box::from(ListNode::new(list1_unwraped.val));
                next.next = Solution::merge_two_lists(list1_unwraped.next, None);
                Some(next)
            }
            (None, Some(list2_unwraped)) => {
                let mut next = Box::from(ListNode::new(list2_unwraped.val));
                next.next = Solution::merge_two_lists(list2_unwraped.next, None);
                Some(next)
            }
            _ => None,
        }
    }
}
