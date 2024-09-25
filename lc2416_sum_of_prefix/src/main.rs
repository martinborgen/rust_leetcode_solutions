impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut output = Vec::with_capacity(words.len());

        for word in &words {
            let mut prefixes: Vec<String> = Vec::with_capacity(word.len());
            let mut count = 0;
            for i in 1..word.len() + 1 {
                prefixes.push(word[0..i].into());
            }

            for pref in prefixes {
                for w in &words {
                    if pref.len() <= w.len() && pref == w[0..pref.len()] {
                        count += 1;
                    }
                }
            }

            output.push(count);
        }

        return output;
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::sum_prefix_scores(vec![
            String::from("abc"),
            String::from("ab"),
            String::from("bc"),
            String::from("b"),
        ]),
        vec![5, 4, 3, 2]
    );

    assert_eq!(
        Solution::sum_prefix_scores(vec![String::from("abcd")]),
        vec![4]
    )
}
