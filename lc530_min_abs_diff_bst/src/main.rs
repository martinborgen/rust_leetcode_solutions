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
      right: None
    }
  }
}



use std::rc::Rc;
use std::cell::RefCell;
// impl Solution {
//     pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
//     }
// }


struct Solution;

fn tree_init_from_vec(data: Vec<Option<i32>>) -> Rc<RefCell<TreeNode>> {
    let mut root = Rc::new(RefCell::new(TreeNode::new(data[0].unwrap())));
    for i in &data[1..] {
        match i {
            Some(n) => root = tree_push(&Some(root.clone()), *n),
            None => continue
        };
    }
    return root;
}

fn tree_push(root_opt: &Option<Rc<RefCell<TreeNode>>>, data: i32) -> Rc<RefCell<TreeNode>> {
    let root: Rc<RefCell<TreeNode>>;
    match root_opt {
        Some(ref x) => root = x.to_owned(),
        None => return Rc::new(RefCell::new(TreeNode::new(data))),
    }
    
    let root_val = root.try_borrow().unwrap().val; // we know root exists now, so it must have a val
    if data == root_val {
        return root_opt.clone().unwrap();
    } else if data < root_val {
        let tmp = tree_push(&root.borrow().left, data);
        root.borrow_mut().left = Some(tmp);
    } else {
        let tmp = tree_push(&root.borrow().right, data);
        root.borrow_mut().right = Some(tmp);
    }

    return root;
}

fn main() {
    let test_vec: Vec<Option<i32>> = vec![Some(1),Some(0),Some(48),None,None,Some(12),Some(49)];
    let tree = tree_init_from_vec(test_vec);
    println!("hello kek");
}
