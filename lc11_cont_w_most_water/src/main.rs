// Container with most water
// Martin Borgén
// 2023-06-02

use std::cmp::min;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut largest = 0;

        for i in 0..height.len() {
            let hi = height[i];

            for j in (i..height.len()).rev() {
                let hj = height[j];

                let area = min(hi, hj) * (j - i) as i32;
                if area > largest {
                    largest = area;
                }

                if hj >= hi {
                    break;
                }
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
}
