// LeetCode problem 2719
// Martin Borgén
// 2023-06-04

impl Solution {
    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        let base: i32 = 10;
        let max = base.pow(9) + 7;

        if 9 * (num2.len() as i32) < min_sum {
            return 0;
        } else if Self::digit_sum_str(&num2, max) < 0 {
            
        }

        let n1 = i32::from_str_radix(&num1, 10).unwrap();
        let n2 = i32::from_str_radix(&num2, 10).unwrap();

        if 9 * (num2.len() as i32) < max_sum && Self::digit_sum(n1, n2) > min_sum {
            return (n2 - n1 + 1) % max;
        }
        
        let mut count = 0;
        for i in n1..n2+1 {
            let res = Self::digit_sum(i, max_sum);
            if res >= min_sum {
                count += 1;
            }
        }

        return count % max;
    }

    fn digit_sum(num: i32, max: i32) -> i32 {
        let mut sum = 0;
        let mut working = num;
        while working > 0 {
            sum += working % 10;
            working /= 10;

            if sum > max {
                return -1;
            }
        }
        return sum;
    }

    fn digit_sum_str(num: &str, max: i32) -> i32 {
        let mut sum = 0;
        for c in num.chars() {
            sum += i32::from_str_radix(&c.to_string(), 10).unwrap();
            if sum >= max {
                return -1;
            }
        }
        return sum;
    }
}


struct Solution;

fn main() {
    // assert_eq!(Solution::digit_sum(1023, 6), 6);
    assert_ne!(Solution::digit_sum_str(&"1341489267173864014004".to_string(), 204),  204);
    assert_eq!(Solution::count("1".to_string(), "12".to_string(), 1, 8), 11);
    assert_eq!(Solution::count("1".to_string(), "5".to_string(), 1, 5), 5);
    assert_eq!(Solution::count("1000000007".to_string(), "2000000014".to_string(), 1, 400), 1);
    assert_eq!(Solution::count("332".to_string(), "863".to_string(), 20, 29), 61);
    assert_eq!(Solution::count("862662375825717340797".to_string(), "1341489267173864014004".to_string(), 204, 338), 0);
    assert_eq!(Solution::count("1".to_string(), "10000000000000000000000".to_string(), 1, 400), 1);
}   
