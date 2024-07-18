impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }

        let specials = "!@#$%^&*()-+";

        let mut has_lower = false;
        let mut has_upper = false;
        let mut has_digit = false;
        let mut has_special = false;

        let mut prev = ' '; // space is not allowed it seems.

        for c in password.chars() {
            if c.is_lowercase() {
                has_lower = true;
            }

            if c.is_uppercase() {
                has_upper = true;
            }

            if specials.contains(c) {
                has_special = true;
            }

            if c.is_numeric() {
                has_digit = true;
            }

            if c == prev {
                return false;
            } else {
                prev = c;
            }
        }

        has_lower && has_upper && has_digit && has_special
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::strong_password_checker_ii(String::from("IloveLe3tcode!")), true);
    assert_eq!(Solution::strong_password_checker_ii(String::from("Me+You--IsMyDream")), false);
    assert_eq!(Solution::strong_password_checker_ii(String::from("1aB!")), false);
}
