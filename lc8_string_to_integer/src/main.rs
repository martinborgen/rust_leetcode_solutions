
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut output = 0;
        let mut pos = true;
        for c in s.trim().chars() {
            // Check sign
            if c == '-' {
                pos = false;
                continue;
            } else if c == '+' {
                continue;
            }

            // remove leading zeros
            if (c as u32 - '0' as u32) == 0 {
                continue;
            }

            // read digits
            if (c as u32 - '0' as u32) <= 9 {
                output *= 10;
                output += c as u32 - '0' as u32;
            } else {
                break;
            }
        }
        
        if pos {
            return output as i32;
        } else {
            return -(output as i32);
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
}
