impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;
        let mut mid: usize;
        if nums[left] < nums [right] {
            // means no shift has occurred
            return nums[left];
        }
        
        while left < right {
            mid = (right + left) / 2;
            if nums[mid] > nums [right] {
                // means shift is in right half
                left = mid + 1;
            } else {
                // means hsift is in left half
                right = mid;
            }
        }
        return nums[right];
    }
}


struct Solution;

fn main() {
    assert_eq!(Solution::find_min(vec![3,4,5,1,2]), 1);
    assert_eq!(Solution::find_min(vec![4,5,6,7,0,1,2]), 0);
    assert_eq!(Solution::find_min(vec![11,13,15,17]), 11);
    assert_eq!(Solution::find_min(vec![4,5,0,1,2,3]), 0);
    assert_eq!(Solution::find_min(vec![4,5,6,7,2,3]), 2);
    assert_eq!(Solution::find_min(vec![3,1,2]), 1);
}