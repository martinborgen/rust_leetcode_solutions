
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut output = 0;
        let mut pos = true;
        let mut sign = false;
        for c in s.trim().chars() {
            // Check sign
            if c == '-' && !sign{
                pos = false;
                sign = true;
                continue;
            } else if c == '+' && !sign{
                sign = true;
                continue;
            }
            let d = c as i32 - '0' as i32;
            
            // remove leading zeros
            if d == 0 && output > 0 {
                sign = true;
                continue;
            }

            // read digits. Working in negative to handle i32::MIN and MAX
            if d >= 0 && d <= 9 {
                sign = true;
                if output >= i32::MIN / 10 {
                    output *= 10;
                } else {
                    output = i32::MIN;
                    break;
                } 

                if output > i32::MIN + d {
                    output -= d;
                } else {
                    output = i32::MIN;
                    break;
                }
            } else {
                break;
            }
        }
        
        if pos {
            if output == i32::MIN {
                return i32::MAX;
            } else {
                return -(output as i32);
            }
        } else {
            return output as i32;
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::my_atoi(String::from("42")), 42);
    assert_eq!(Solution::my_atoi(String::from("+42")), 42);
    assert_eq!(Solution::my_atoi(String::from("-42")), -42);
    assert_eq!(Solution::my_atoi(String::from("0042")), 42);
    assert_eq!(Solution::my_atoi(String::from("-0042")), -42);
    assert_eq!(Solution::my_atoi(String::from("asdfgasd")), 0);
    assert_eq!(Solution::my_atoi(String::from("-asdf")), 0);
    assert_eq!(Solution::my_atoi(String::from("42asdf")), 42);
    assert_eq!(Solution::my_atoi(String::from("-42asdf")), -42);
    assert_eq!(Solution::my_atoi(String::from("-asdf42")), 0);
    assert_eq!(Solution::my_atoi(String::from("   42")), 42);
    assert_eq!(Solution::my_atoi(String::from("   -42")), -42);
    assert_eq!(Solution::my_atoi(String::from("42 with words")), 42);
    assert_eq!(Solution::my_atoi(String::from("2147483646")), 2147483646);
    assert_eq!(Solution::my_atoi(String::from("2147483647")), 2147483647);
    assert_eq!(Solution::my_atoi(String::from("2147483649")), 2147483647);
    assert_eq!(Solution::my_atoi(String::from("-2147483648")), -2147483648);
    assert_eq!(Solution::my_atoi(String::from("-2147483649")), -2147483648);
    assert_eq!(Solution::my_atoi(String::from("-")), 0);
    assert_eq!(Solution::my_atoi(String::from("")), 0);
    assert_eq!(Solution::my_atoi(String::from("+-12")), 0);
    assert_eq!(Solution::my_atoi(String::from("00000-42a1234")), 0);
    assert_eq!(Solution::my_atoi(String::from("010")), 10);
}
