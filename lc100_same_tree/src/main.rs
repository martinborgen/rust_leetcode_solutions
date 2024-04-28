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
      right: None
    }
  }
}

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
  pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack: VecDeque<Option<Rc<RefCell<TreeNode>>>> = Default::default();
    stack.push_back(p);
    while stack.len() > 0 {
      let pp = stack.pop_front().unwrap();
      if pp.is_some() {
        let current = pp.unwrap();
        println!("{}", current.borrow_mut().val);
        stack.push_back(current.borrow_mut().left.clone());
        stack.push_back(current.borrow_mut().right.clone());
      }

    }
    return true;
  }
}

struct Solution;

fn make_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
  if vec.len() == 0 || vec[0].is_none() {
      return None;
  }
  let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {val: vec[0].unwrap(), left: None, right: None})));
  let mut root_ref: Option<Rc<RefCell<TreeNode>>> = None; // This is just to re-catch the root to circumvent the damn borrow checker
  let mut stack: VecDeque<Option<Rc<RefCell<TreeNode>>>> = Default::default();
  stack.push_back(root);
  
  let mut i = 1;
  while i < vec.len() {
    let current: Rc<RefCell<TreeNode>> = stack.pop_front().unwrap().unwrap();
    if vec[i].is_some() {
      current.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode {val: vec[i].unwrap(), left: None, right: None})));
      stack.push_back(current.borrow_mut().left.clone());
    }
    if i + 1 < vec.len() && vec[i+1].is_some() {
      current.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode {val: vec[i + 1].unwrap(), left: None, right: None})));
      stack.push_back(current.borrow_mut().right.clone());
    }
    if i == 1 {
      root_ref = Some(current); // explicitly handing root back really necessary?
    }
    i += 2;
  }
  return root_ref;
}

fn main() {
  let a = TreeNode {val:1, left: None, right: None};
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

  let tst = make_tree(vec!{Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)});
  Solution::is_same_tree(tst, Some(a_rc));

}
