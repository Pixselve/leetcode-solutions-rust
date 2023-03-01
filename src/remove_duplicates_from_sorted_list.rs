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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head_2 = head;
        head_2.as_ref()?;
        let mut pointer_1 = head_2.as_mut().unwrap();
        while let Some(pointer_2) = pointer_1.next.as_mut() {
            if pointer_1.val == pointer_2.val {
                pointer_1.next = pointer_2.next.take();
            } else if pointer_1.next.is_some() {
                pointer_1 = pointer_1.next.as_mut().unwrap();
            }
        }
        head_2
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test() {
        let list = ListNode { val: 1, next: Some(Box::from(ListNode { val: 1, next: Some(Box::from(ListNode { val: 1, next: None })) })) };
        assert_eq!(Solution::delete_duplicates(Some(Box::from(list))), Some(Box::from(ListNode::new(1))));
    }
}
