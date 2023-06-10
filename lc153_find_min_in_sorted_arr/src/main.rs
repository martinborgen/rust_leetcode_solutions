impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut mid: usize = nums.len() / 2;
        let mut right: usize = nums.len() - 1;

        if nums[left] < nums [right] {
            // means no shift has occurred
            return nums[left];
        }
        
        loop {
            // first checking if we've found a minimum
            if mid == right {
                return nums[mid];
            } else if mid == left {
                return nums [right];
            }
            
            // deciding where to go
            if nums[mid] > nums [right] {
                // means shift is in right half
                left = mid;
                mid = mid + (right - mid) / 2;
                // continue;
            } else {
                // means hsift is in left half
                right = mid;
                mid = mid - (mid - left) / 2;
                // continue;
            }

        }
    }
}


struct Solution;

fn main() {
    assert_eq!(Solution::find_min(vec![3,4,5,1,2]), 1);
    assert_eq!(Solution::find_min(vec![4,5,6,7,0,1,2]), 0);
    assert_eq!(Solution::find_min(vec![11,13,15,17]), 11);
    assert_eq!(Solution::find_min(vec![4,5,0,1,2,3]), 0);
    assert_eq!(Solution::find_min(vec![4,5,6,7,2,3]), 2);
}