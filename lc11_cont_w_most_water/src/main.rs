// Container with most water
// Martin Borgén
// 2023-06-02

use std::cmp::min;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut largest = 0;
        let mut hi_max = 0;

        for i in 0..height.len() {
            let hi = height[i];
            if hi > hi_max {
                hi_max = hi;
            } else {
                continue;
            }

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
    assert_eq!(Solution::max_area(vec![1,832,6,2,5,4,8,3,7,1,2,4,4,6,456,23,76,3,32,23,4,5,5,64,546,7,2,345,123,74,432]), 12558);
    assert_eq!(Solution::max_area(vec![1,832,6,2,5,4,8,3,10000,1,2,4,4,6,456,23,76,3,32,23,4,5,5,64,546,7,2,345,123,74,432,1,2,3,2,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,10000,3]), 600000);
}

// Better slution idea here:
// impl Solution {
//     pub fn max_area(height: Vec<i32>) -> i32 {
//         let (mut start, mut end) = (0, height.len() - 1);
//         let mut biggest_area = 0;
//         while start != end {
//             let area = (end - start) as i32 * i32::min(height[start], height[end]);
//             if height[start] > height[end] {
//                 end -= 1;
//             }
//             else {
//                 start += 1;
//             }
//             biggest_area = i32::max(area, biggest_area)
//         }
//         biggest_area
//     }
// }