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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
  pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack: Vec<TreeNode> = vec![];
    match (p, q) {
      (Some(p), Some(q)) => true,
      // (None(p), None(q)) => true,
      _ => false,
    }
  }
}

struct Solution;

fn make_tree(vec: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
  if vec[0] == 0 {
    return None;
  }
  let root = Some(Rc::new(RefCell::new(TreeNode {val: vec[0], left: None, right: None})));
  let mut stack: Vec<&Option<Rc<RefCell<TreeNode>>>> = vec![&root];
  let mut current = root.as_ref().unwrap();
  for i in &vec[1..] {
    
  }
  
  return root;
}

fn main() {
  let a = TreeNode {val:1, left: None, right: None};
  let a_ref_cell = RefCell::new(a);
  let a_rc = Rc::new(a_ref_cell);

  let b = TreeNode {val: 1, left: None, right: None};
  let b_ref_cell = RefCell::new(b);
  let b_rc = Rc::new(b_ref_cell);

  let c = TreeNode {val: 1, left: None, right: Some(a_rc)};
  let c_ref_cell = RefCell::new(c);
  let c_rc = Rc::new(c_ref_cell);

  let d = TreeNode {val: 1, left: None, right: Some(b_rc)};
  let d_ref_cell = RefCell::new(d);
  let d_rc = Rc::new(d_ref_cell);

  let tst = d_rc == c_rc;

  // let tst = Solution::is_same_tree(Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None}))), Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: None}))));
  match tst {
      true => println!("True"),
      false => println!("False"),
  }
}
