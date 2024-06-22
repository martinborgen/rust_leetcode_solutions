// Leetcode problem 530
// Return minimum absolute difference in a BST
// Martin Borgén
// 20203-06-15

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::cmp::min;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let left;
        let right;
        let left_res: i32;
        let right_res: i32;
        let diff_left_right: i32;
        let diff_root_left: i32;
        let diff_root_right: i32;
        match root {
            Some(root) => {
                left = root.borrow().left.clone();
                right = root.borrow().right.clone();
                match (left, right) {
                    (Some(left), Some(right)) => {
                        left_res = i32::abs(Self::get_minimum_difference(Some(left.clone())));
                        right_res = i32::abs(Self::get_minimum_difference(Some(right.clone())));
                        diff_left_right = i32::abs(left.borrow().val - right.borrow().val);
                        diff_root_left = i32::abs(left.borrow().val - root.borrow().val);
                        diff_root_right = i32::abs(right.borrow().val - root.borrow().val);
                        return i32::abs(min(
                            diff_left_right,
                            min(
                                min(left_res, right_res),
                                min(diff_root_left, diff_root_right),
                            ),
                        ));
                    }
                    (None, None) => return i32::MAX,
                    (None, Some(right)) => {
                        right_res = i32::abs(Self::get_minimum_difference(Some(right.clone())));
                        return min(i32::abs(right.borrow().val - root.borrow().val), right_res);
                    }
                    (Some(left), None) => {
                        left_res = i32::abs(Self::get_minimum_difference(Some(left.clone())));
                        return min(i32::abs(left.borrow().val - root.borrow().val), left_res);
                    }
                }
            }
            None => i32::MAX,
        }
    }
}

struct Solution;

fn make_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if vec.is_empty() || vec[0].is_none() {
        return None;
    }
    let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
        val: vec[0].unwrap(),
        left: None,
        right: None,
    })));
    let mut output: Option<Rc<RefCell<TreeNode>>> = None; // This is just to re-catch the root to circumvent the damn borrow checker
    let mut stack: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
    stack.push_back(root);

    let mut i = 1;
    while i < vec.len() {
        let popped = stack.pop_front();
        if let Some(Some(current)) = popped {
            if let Some(x) = vec[i] {
                current.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode {
                    val: x,
                    left: None,
                    right: None,
                })));
                stack.push_back(current.borrow_mut().left.clone());
            }

            if i + 1 < vec.len() {
                if let Some(y) = vec[i + 1] {
                    current.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode {
                        val: y,
                        left: None,
                        right: None,
                    })));
                    stack.push_back(current.borrow_mut().right.clone());
                }
            }

            if i == 1 {
                output = Some(current); // explicitly handing back root. Gotta be a better way?
            }

            i += 2;
        }
    }
    output
}

fn main() {
    let test_vec1: Vec<Option<i32>> =
        vec![Some(1), Some(0), Some(48), None, None, Some(12), Some(49)];
    let tree1 = make_tree(test_vec1);

    let res1 = Solution::get_minimum_difference(tree1);
    assert_eq!(res1, 1);

    let test_vec2: Vec<Option<i32>> = vec![Some(4), Some(2), Some(6), Some(1), Some(3)];
    let tree2 = make_tree(test_vec2);
    let res2 = Solution::get_minimum_difference(tree2);
    assert_eq!(res2, 1);

    let test_vec3: Vec<Option<i32>> = vec![Some(1), None, Some(3), Some(2)];
    let tree3 = make_tree(test_vec3);
    let res3 = Solution::get_minimum_difference(tree3);
    assert_eq!(res3, 1);

    let test_vec4: Vec<Option<i32>> = vec![
        Some(236),
        Some(104),
        Some(701),
        None,
        Some(227),
        None,
        Some(911),
    ];
    let tree4 = make_tree(test_vec4);
    let res4 = Solution::get_minimum_difference(tree4);
    assert_eq!(res4, 9);
}
