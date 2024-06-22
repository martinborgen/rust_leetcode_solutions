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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let left_res;
        let right_res;
        let diff_left_right: i32;
        let diff_root_left: i32;
        let diff_root_right: i32;
        match root {
            Some(root) => {
                left_res = Self::get_minimum_difference(root.borrow().left.clone());
                right_res = Self::get_minimum_difference(root.borrow().right.clone());
                if left_res < right_res {
                    return left_res;
                } else {
                    return right_res;
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
    let test_vec: Vec<Option<i32>> =
        vec![Some(1), Some(0), Some(48), None, None, Some(12), Some(49)];
    let tree = make_tree(test_vec);

    let res = Solution::get_minimum_difference(tree);
    println!("{res}");
}
