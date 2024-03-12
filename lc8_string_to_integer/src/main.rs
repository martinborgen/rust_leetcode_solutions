
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
            if (c as i32 - '0' as i32) == 0 {
                continue;
            }

            // read digits
            if (c as i32 - '0' as i32) >= 0 && (c as i32 - '0' as i32) <= 9 {
                if output > std::i32::MIN / 10 {
                    output *= 10;
                } else {
                    output = std::i32::MIN; // Missar fortfarande int MIN?
                    break;
                } 

                if output + (c as i32 - '0' as i32) > std::i32::MIN {
                    output -= c as i32 - '0' as i32;
                } else {
                    output = std::i32::MIN;
                    break;
                }
            } else {
                break;
            }
        }
        
        if pos {
            if output == std::i32::MIN {
                return std::i32::MAX;
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
    assert_eq!(Solution::my_atoi(String::from("2147483647")), 2147483647);
    assert_eq!(Solution::my_atoi(String::from("2147483649")), 2147483647);
    assert_eq!(Solution::my_atoi(String::from("-2147483648")), -2147483648);
    assert_eq!(Solution::my_atoi(String::from("-2147483649")), -2147483648);
}
