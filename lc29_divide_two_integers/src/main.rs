impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        // Handle edge cases with i32::MAX or MIN, and divisor = +-1
        if divisor == 1 {
            return dividend;
        } else if divisor == -1 && dividend == i32::MIN {
            return i32::MAX;
        } else if divisor == -1 {
            return !dividend + 1;
        } else if divisor == 2 {
            return dividend >> 1;
        } else if divisor == -2 {
            return !(dividend >> 1);
        }

        // Set same sign on both
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

        // Check so that loop can begin with quotient == 1
        if divisor < dividend {
            return 0;
        }

        let mut quotient = 1;
        let mut tmp = divisor;
        let mut shifts = 0;
        let cutoff = dividend >> 1;
        while tmp > cutoff {
            tmp <<= 1;
            quotient <<= 1;
            shifts += 1;
        }

        dividend -= tmp;
        while shifts >= 0 {
            if tmp >= dividend {
                dividend -= tmp;
                quotient += 1 << shifts;
            }

            tmp >>= 1;
            shifts -= 1;
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
    assert_eq!(Solution::divide(-231, 3), -77);
    assert_eq!(Solution::divide(10, 3), 3);
    assert_eq!(Solution::divide(7, -3), -2);
    assert_eq!(Solution::divide(2, 2), 1);
    assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
    assert_eq!(Solution::divide(25, 2), 12);
    assert_eq!(Solution::divide(0, i32::MAX), 0);
    assert_eq!(Solution::divide(i32::MIN, i32::MIN), 1);
    assert_eq!(Solution::divide(i32::MAX, i32::MAX), 1);
    assert_eq!(Solution::divide(-25, 3), -8);
    assert_eq!(Solution::divide(i32::MIN, i32::MAX), -1);
    assert_eq!(Solution::divide(-2147483648, 2), -1073741824);
    assert_eq!(Solution::divide(-2147483648, 4), -536870912);
}
