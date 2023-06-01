// Container with most water
// Martin Borgén
// 2023-06-02

use std::cmp::min;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut largest = 0;
        let mut left: usize = 0;
        let mut right = height.len() - 1;
        
        while right != left {
            let hl = height[left];
            let hr = height[right];
            let area = min(hr, hl) * (right - left) as i32;
            if area > largest {
                largest = area;
            }
            
            if hl < hr {
                left += 1;
            } else {
                right -= 1;
            }
        }
        return largest;
    }
}
struct Solution;

fn main() {
    assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    assert_eq!(Solution::max_area(vec![1,1]), 1);
    assert_eq!(Solution::max_area(vec![10000; 100000]), 999990000);
    assert_eq!(Solution::max_area(vec![1,832,6,2,5,4,8,3,7,1,2,4,4,6,456,23,76,3,32,23,4,5,5,64,546,7,2,345,123,74,432]), 12558);
    assert_eq!(Solution::max_area(vec![1,832,6,2,5,4,8,3,10000,1,2,4,4,6,456,23,76,3,32,23,4,5,5,64,546,7,2,345,123,74,432,1,2,3,2,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,10000,3]), 600000);
    assert_eq!(Solution::max_area(vec![1,10,1,2,3,4,3,2,1,300,10,2]), 90);
}
