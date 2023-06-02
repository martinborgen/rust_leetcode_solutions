// LC 41 Merge K sorted (linked) lists
// Martin Borgén
// 2023-06-02

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut out = None;
        for list in lists {
            out = Self::merge_2_lists(list, out);
        }
        return out;
    }

    fn merge_2_lists(list_a: Option<Box<ListNode>>, list_b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Basically stolen solution. I know the algorithm, but not the Rust implementation.
        // Commenting here so I'm sure I understand everything. /M
        
        let mut out = None;         // None necessary to initialize so that out_next can point to it's location? 

        let mut out_next = & mut out; // movable pointer so that we can assign stuff here.
        let mut a_opt = list_a;     // var to current a pos
        let mut b_opt = list_b;     // -||-           b pos

        loop {
            // first we 'unSome' the a_opt. If none, we return b_opt as is.
            let mut a = match a_opt {
                Some(a) => a,
                None => {*out_next = b_opt; break;}
            };

            // Next we 'unSome' the b_opt. If None, we 're-Some' a and return it. 
            let mut b = match b_opt {
                Some(b) => b,
                None => {*out_next = Some(a); break;}
            };

            if a.val < b.val {
                a_opt = a.next.take(); // take is the method that allows a_opt to not keep ownership of a?
                b_opt = Some(b);       // re-Some b as is
                *out_next = Some(a);   // re-Some a now that a.next is taken. 
            } else {                   // vice-versa
                a_opt = Some(a);
                b_opt = b.next.take();
                *out_next = Some(b);
            }

            out_next = &mut out_next.as_mut().unwrap().next; // set out_next to a &mut of out_next.next
        }

        out
    }

}


struct Solution;

fn list_string(list: Option<Box<ListNode>>) -> String {
    let mut out = String::new();
    out.insert(0, '[');
    let mut curr_opt = &list;
    
    while curr_opt.is_some() {
        let curr = curr_opt.as_ref().unwrap();
        out.push_str(&curr.val.to_string());
        out.push(',');
        curr_opt = &curr.next;
    }
    out.pop();
    out.push(']');

    return out;
}

fn linked_list_from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut out_next = &mut head;
    for val in vec {
        *out_next = Some(Box::new(ListNode::new(val)));
        out_next = &mut out_next.as_mut().unwrap().next;
    }
    head
}

fn main() {
    assert_eq!(list_string(Solution::merge_k_lists(vec![linked_list_from_vec(vec![1,4,5]),
                                            linked_list_from_vec(vec![1,3,4]),
                                            linked_list_from_vec(vec![2,6])])), String::from("[1,1,2,3,4,4,5,6]"));
}
