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
        let mut list_1_val = list_1.val;
        let mut list_2_val = list_2.val;

        let first_val = list_1_val + list_2_val;
        let mut out = Box::new(ListNode::new(first_val % 10));
        let mut out_latest = &mut out;

        let mut old_carry = first_val / 10;

        while (old_carry > 0 && (list_1.next.is_some() || list_2.next.is_some()))
            || (list_1.next.is_some() && list_2.next.is_some())
        {
            list_1 = match list_1.next {
                Some(node) => {
                    list_1_val = node.val;
                    node
                }
                None => {
                    list_1_val = 0;
                    list_1
                }
            };
            list_2 = match list_2.next {
                Some(node) => {
                    list_2_val = node.val;
                    node
                }
                None => {
                    list_2_val = 0;
                    list_2
                }
            };

            let sum = list_1_val + list_2_val + old_carry;
            let val = sum % 10;
            let carry = sum / 10;

            let new_next = Box::new(ListNode::new(val));
            out_latest.next = Some(new_next.clone());

            old_carry = carry;
            out_latest = out_latest.next.as_mut().unwrap(); // unwrap because we just added the next
        }

        // by this point just 'append' whatever's left
        if old_carry > 0 {
            out_latest.next = Some(Box::new(ListNode::new(old_carry)));
        }

        if list_1.next.is_some() {
            out_latest.next = list_1.next;
        }

        if list_2.next.is_some() {
            out_latest.next = list_2.next;
        }

        Some(out)
    }
}

fn vec_to_reversed_linked_list(v: Vec<i32>) -> Box<ListNode> {
    let mut v_iter = v.iter().rev();
    let mut out = Box::new(ListNode::new(*v_iter.next().unwrap()));

    let mut out_current = &mut out;

    for i in v_iter {
        out_current.next = Some(Box::new(ListNode::new(*i)));

        out_current = out_current.next.as_mut().unwrap();
    }

    out
}

fn linked_list_to_vec(node: Box<ListNode>) -> Vec<i32> {
    let mut out = vec![node.val];

    let mut n = &node;
    while n.next.is_some() {
        n = n.next.as_ref().unwrap();
        out.push(n.val);
    }

    out
}

fn main() {
    let l1 = vec![9, 9, 9, 9, 9, 9, 9];
    let l2 = vec![9, 9, 9, 9];

    let ll1 = vec_to_reversed_linked_list(l1);
    let ll2 = vec_to_reversed_linked_list(l2);

    let res = Solution::add_two_numbers(Some(ll1), Some(ll2));
    assert!(res.is_some());

    let resvec = linked_list_to_vec(res.unwrap());
    assert_eq!(resvec, vec![8, 9, 9, 9, 0, 0, 0, 1]);
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

    #[test]
    fn test_vec_helper_fun() {
        let testvec = vec![1, 2, 3, 4];
        let res = vec_to_reversed_linked_list(testvec);

        assert_eq!(res.val, 4);
        assert_eq!(res.next.unwrap().val, 3);

        let mut test_linked_list = ListNode::new(2);
        test_linked_list.next = Some(Box::new(ListNode::new(4)));
        test_linked_list.next.as_mut().unwrap().as_mut().next = Some(Box::new(ListNode::new(3)));
        let res_2 = linked_list_to_vec(Box::new(test_linked_list));
        assert_eq!(res_2, vec![2, 4, 3]);
    }

    #[test]
    fn test_advanced_inputs() {
        let l1 = vec![9, 9, 9, 9, 9, 9, 9];
        let l2 = vec![9, 9, 9, 9];

        let ll1 = vec_to_reversed_linked_list(l1);
        let ll2 = vec_to_reversed_linked_list(l2);

        let res = Solution::add_two_numbers(Some(ll1), Some(ll2));
        assert!(res.is_some());

        let resvec = linked_list_to_vec(res.unwrap());
        assert_eq!(resvec, vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }

    #[test]
    fn failed_input_1() {
        let l1 = vec![1, 8];
        let l2 = vec![0];

        let ll1 = vec_to_reversed_linked_list(l1);
        let ll2 = vec_to_reversed_linked_list(l2);

        let res = Solution::add_two_numbers(Some(ll1), Some(ll2));
        assert!(res.is_some());

        let resvec = linked_list_to_vec(res.unwrap());
        assert_eq!(resvec, vec![8, 1]);
    }
}
