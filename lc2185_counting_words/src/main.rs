impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut count = 0;
        for s in words {
            if s.len() < pref.len() {
                continue;
            }

            if s[..pref.len()] == pref {
                count += 1;
            }
        }
        count
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::prefix_count(
            vec![
                String::from("pay"),
                String::from("attention"),
                String::from("practice"),
                String::from("attend")
            ],
            String::from("at")
        ),
        2
    );

    assert_eq!(
        Solution::prefix_count(
            vec![
                String::from("leetcode"),
                String::from("win"),
                String::from("loops"),
                String::from("success")
            ],
            String::from("code")
        ),
        0
    );
}
