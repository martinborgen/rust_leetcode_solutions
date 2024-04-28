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
        let mut stack: VecDeque<Option<Rc<RefCell<TreeNode>>>> = Default::default();
        let mut gen = 0;
        let mut count = 0;
        stack.push_back(p);
        while !stack.is_empty() {
            let pp = stack.pop_front().unwrap();
            let nodes_in_gen = i32::pow(2, gen);
            count += 1;

            if let Some(current) = pp {
                print!("{} ", current.borrow_mut().val);
                stack.push_back(current.borrow_mut().left.clone());
                stack.push_back(current.borrow_mut().right.clone());
            }
            if count == nodes_in_gen {
                count = 0;
                gen += 1;
                println!();
            }
        }
        return true;
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
    return true;
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
    return output;
}

fn main() {
    let a = TreeNode {
        val: 1,
        left: None,
        right: None,
    };
    let a_ref_cell = RefCell::new(a);
    let a_rc = Rc::new(a_ref_cell);

    // let b = TreeNode {val: 1, left: None, right: None};
    // let b_ref_cell = RefCell::new(b);
    // let b_rc = Rc::new(b_ref_cell);

    // let c = TreeNode {val: 1, left: None, right: Some(a_rc)};
    // let c_ref_cell = RefCell::new(c);
    // let c_rc = Rc::new(c_ref_cell);

    // let d = TreeNode {val: 1, left: None, right: Some(b_rc)};
    // let d_ref_cell = RefCell::new(d);
    // let d_rc = Rc::new(d_ref_cell);

    let tst = make_tree(vec![
        Some(1),
        Some(2),
        None,
        Some(4),
        Some(5),
        Some(6),
        None,
        Some(10),
        Some(22),
    ]);
    print_tree(tst);
}
