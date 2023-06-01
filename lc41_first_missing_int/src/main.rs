use std::collections::HashSet;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums_hashed = HashSet::with_capacity(nums.len());
        let mut min: i32 = i32::MAX; // to ensure we idenify min
        let mut max: i32 = 0;        // we're only interested in max > 0

        for num in nums {
            if num > 0 && num < min {
                min = num;
            } 
            if num > 0 && num > max {
                max = num;
            }
            nums_hashed.insert(num);
        }

        if min > 1 {
            return 1;
        }

        for i in (min-1)..max {
            // Shifting i one down to avoid overflow when max == INT_MAX
            if !nums_hashed.contains(&(i+1)) {
                return i+1;
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
}
