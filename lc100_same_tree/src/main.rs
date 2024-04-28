use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p.borrow(), q.borrow()) {
            (Some(p), Some(q)) => {
                let l = Solution::is_same_tree(
                    p.borrow_mut().left.clone(),
                    q.borrow_mut().left.clone(),
                );
                let r = Solution::is_same_tree(
                    p.borrow_mut().right.clone(),
                    q.borrow_mut().right.clone(),
                );
                l && r && (p == q)
            }
            _ => p == q,
        }
    }
}

fn print_tree(p: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
    let mut gen = 0;
    let mut count = 0;
    queue.push_back(p);
    while !queue.is_empty() {
        let pp = queue.pop_front().unwrap();
        let nodes_in_gen = i32::pow(2, gen);
        count += 1;

        if let Some(current) = pp {
            print!("{} ", current.borrow_mut().val);
            queue.push_back(current.borrow_mut().left.clone());
            queue.push_back(current.borrow_mut().right.clone());
        }
        if count == nodes_in_gen {
            count = 0;
            gen += 1;
            println!();
        }
    }
    true
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
    assert!(
        Solution::is_same_tree(
            make_tree(vec![Some(1), Some(2), Some(3)]),
            make_tree(vec![Some(1), Some(2), Some(3)])
        )
    );
    assert!(
        !Solution::is_same_tree(
            make_tree(vec![Some(1), Some(2)]),
            make_tree(vec![Some(1), None, Some(3)])
        )
    );
    assert!(
        !Solution::is_same_tree(
            make_tree(vec![Some(1), Some(2), Some(1)]),
            make_tree(vec![Some(1), Some(1), Some(2)])
        )
    );
}
