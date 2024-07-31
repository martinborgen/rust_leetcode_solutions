impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        if divisor == 1 {
            return dividend;
        } else if divisor == -1 && dividend == i32::MIN {
            return i32::MAX;
        } else if divisor == -1 {
            return !dividend + 1;
        } 
        let mut neg = 0;
        if divisor > 0 {
            divisor = !divisor + 1;
        } else {
            neg += 1;
        }

        if dividend > 0 {
            dividend = !dividend + 1;
        } else {
            neg += 1;
        }

        let mut quotient = 0;
        while dividend <= divisor {
            dividend -= divisor;
            quotient += 1;
        }
        
        if neg == 1 {
            return !quotient + 1;
        } else {
            return quotient;
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::divide(10, 3), 3);
    assert_eq!(Solution::divide(7, -3), -2);
    assert_eq!(Solution::divide(2, 2), 1);
    assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
}
