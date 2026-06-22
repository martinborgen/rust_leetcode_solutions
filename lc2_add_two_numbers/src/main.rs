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

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // problem guarantees the lists are non-zero
        let mut list_1 = l1.unwrap();
        let mut list_2 = l2.unwrap();

        let first_val = list_1.val + list_2.val;
        let mut out = Box::new(ListNode::new(first_val % 10));
        let mut out_latest = &mut out;

        let mut old_carry = first_val / 10;

        while list_1.next.is_some() && list_2.next.is_some() {
            list_1 = list_1.next.unwrap();
            list_2 = list_2.next.unwrap();

            let sum = list_1.val + list_2.val;
            let val = sum % 10;
            let carry = sum / 10;

            let new_next = Box::new(ListNode::new(val + old_carry));
            out_latest.next = Some(new_next.clone());

            old_carry = carry;
            out_latest = out_latest.next.as_mut().unwrap(); // unwrap because we just added the next
        }

        Some(out)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input() {
        let mut l1 = ListNode::new(2);
        l1.next = Some(Box::new(ListNode::new(4)));
        l1.next.as_mut().unwrap().as_mut().next = Some(Box::new(ListNode::new(3)));

        let mut l2 = ListNode::new(5);
        l2.next = Some(Box::new(ListNode::new(6)));
        l2.next.as_mut().unwrap().as_mut().next = Some(Box::new(ListNode::new(4)));

        let out = Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));

        assert!(out.is_some());
        let mut res = Vec::new();

        let mut current = out.clone();
        while current.is_some() {
            res.push(current.as_ref().unwrap().val);
            current = current.unwrap().next;
        }

        assert_eq!(res, vec![7, 0, 8]);
    }
}
