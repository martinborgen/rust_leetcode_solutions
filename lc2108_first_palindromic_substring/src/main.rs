impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words {
            if Self::is_palindrome(&word) {
                return word;
            }
        }
        return String::from("");
    }

    fn is_palindrome(word: &String) -> bool {
        let mut forewards = word.chars();
        let mut backwards = word.chars().rev();

        for _ in 0..(word.len() / 2) {
            if forewards.next() != backwards.next() {
                return false;
            }
        }
        return true;
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::first_palindrome(vec![
            String::from("abc"),
            String::from("car"),
            String::from("ada"),
            String::from("racecar"),
            String::from("cool")
        ]),
        String::from("ada")
    );

    assert_eq!(
        Solution::first_palindrome(vec![
            String::from("notapalindrome"),
            String::from("racecar")
        ]),
        String::from("racecar")
    );

    assert_eq!(
        Solution::first_palindrome(vec![
            String::from("def"),
            String::from("ghi")
        ]),
        String::from("")
    );
}
