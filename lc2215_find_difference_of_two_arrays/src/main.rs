// Input: nums1 = [1,2,3], nums2 = [2,4,6]
// Output: [[1,3],[4,6]]

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = vec![Vec::new(); 2];
        for n1 in &nums1 {
            if !nums2.contains(&n1) && !answer[0].contains(&n1) {
                answer[0].push(*n1);
            }
        }

        for n2 in &nums2 {
            if !nums1.contains(&n2) && !answer[1].contains(&n2) {
                answer[1].push(*n2);
            }
        }
        answer
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_difference(vec![1,2,3], vec![2,4,6]), vec![vec![1,3], vec![4,6]]);
}
