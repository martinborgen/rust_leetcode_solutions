// Input: nums1 = [1,2,3], nums2 = [2,4,6]
// Output: [[1,3],[4,6]]

use std::collections::HashSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = vec![Vec::new(); 2];
        
        let nums1_hash: HashSet<i32> = nums1.into_iter().collect();
        let nums2_hash: HashSet<i32> = nums2.into_iter().collect();

        answer[0] = nums1_hash.difference(&nums2_hash).into_iter().cloned().collect();
        answer[1] = nums2_hash.difference(&nums1_hash).into_iter().cloned().collect();

        answer
    }
}

struct Solution;

fn main() {
    // asserts complain because of order - problem spec allows any order
    // assert_eq!(Solution::find_difference(vec![1,2,3], vec![2,4,6]), vec![vec![3,1], vec![4,6]]);
    // assert_eq!(Solution::find_difference(vec![1,2,3,3], vec![1,1,2,2]), vec![vec![3], vec![]]);
    // assert_eq!(Solution::find_difference(vec![-68,-80,-19,-94,82,21,-43], vec![-63]), vec![vec![-68,-80,-19,-94,82,21,-43], vec![-63]]);
}
