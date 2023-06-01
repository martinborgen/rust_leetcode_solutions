// use std::collections::HashSet;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut min: i32 = i32::MAX; // to ensure we idenify min
        let mut max: i32 = 0;        // we're only interested in max > 0
        let mut found = 0;
        
        for i in 0..nums.len() {
            let num = nums[i];
            
            if num < 1 {
                continue;
            }

            if num < min {
                min = num;
            } 
            // separate if:s because if only 1 num > 0 => num is both min and max
            if num > max {
                max = num;
            }

            found += 1;
        }

        // if no min found, min default is INT_MAX
        if min > 1 {
            return 1;
        }

        let mut nums_found = vec![0; (found) as usize];

        for num in nums {
            if num < 1 || num > found {
                continue;
            }
            // We already know min is one here
            nums_found[(num - 1) as usize] = num;
        }

        for i in 0..nums_found.len() {
            if nums_found[i] == 0 {
                return (i+1) as i32;
            }
        }

        return max + 1;
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::first_missing_positive(vec![1,2,0]), 3);
    assert_eq!(Solution::first_missing_positive(vec![3,4,-1,1]), 2);
    assert_eq!(Solution::first_missing_positive(vec![7,8,9,11,12]), 1);
    assert_eq!(Solution::first_missing_positive(vec![-1,-7,-2,-5]), 1);
    assert_eq!(Solution::first_missing_positive(vec![]), 1);
    assert_eq!(Solution::first_missing_positive(vec![1,2,3,10,2147483647,9]), 4);
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 3, 7, 8]), 4);
}
