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
            match c {
                'a'..='z' => has_lower = true,
                'A'..='Z' => has_upper = true,
                '0'..='9' => has_digit = true,
                _ if specials.contains(c) => has_special = true,
                _ => return false,
            }
            if c != prev {
                prev = c;
            } else {
                return false;
            }
        }

        has_lower && has_upper && has_digit && has_special
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::strong_password_checker_ii(String::from("aaaa")),
        false
    );

    assert_eq!(
        Solution::strong_password_checker_ii(String::from("IloveLe3tcode!")),
        true
    );
    assert_eq!(
        Solution::strong_password_checker_ii(String::from("Me+You--IsMyDream")),
        false
    );
    assert_eq!(
        Solution::strong_password_checker_ii(String::from("1aB!")),
        false
    );

    assert_eq!(
        Solution::strong_password_checker_ii(String::from("0Aa!a!a!")),
        true
    );

    assert_eq!(
        Solution::strong_password_checker_ii(String::from("11A!A!Aa")),
        false
    );
}
