// Martin Borgén 
// 2023-06-10

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;
        let mut mid: usize;

        if nums[left] < nums [right] {
            // means no shift has occurred
            return nums[left];
        }
        
        while nums[left] >= nums[right] {
            mid = (left + right) / 2;
            if left == mid {
                break;
            }

            if nums[mid] < nums[left] {
                // we have shift in left half
                right = mid;
            } else if nums[mid] > nums[right] {
                // we have shift in right half
                left = mid;
                // mid = (left + right) / 2;
            } else if nums[left] == nums[right] {
                // left and right number are same and we have no idea if there even is a shift. 
                while nums[left] == nums[right] && right-1 > left {
                    left += 1;
                    right -= 1;
                }
            }
        }
        
        if nums[left] < nums[right] {
            return nums[left];
        } else {
            return nums[right];
        }
    }
}


struct Solution;

fn main() {
    assert_eq!(Solution::find_min(vec![1,3,5]), 1);
    assert_eq!(Solution::find_min(vec![2,2,2,0,1]), 0);
    assert_eq!(Solution::find_min(vec![3,3,1,3]), 1);
    assert_eq!(Solution::find_min(vec![4,4,1,3,3,3,3]), 1);
    assert_eq!(Solution::find_min(vec![3,3,1,3,3,3,3]), 1);
    assert_eq!(Solution::find_min(vec![5,5,1,2,5,5,5]), 1);
    assert_eq!(Solution::find_min(vec![1,1,1]), 1);
    assert_eq!(Solution::find_min(vec![-1,-1,-1,-1]), -1);
}
