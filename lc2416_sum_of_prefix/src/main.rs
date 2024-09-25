use std::collections::HashMap;

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut output = Vec::with_capacity(words.len());
        let mut hmap: HashMap<String, i32> = HashMap::new();

        for word in &words {
            let mut prefixes: Vec<String> = Vec::with_capacity(word.len());
            let mut i_count = 0;

            for i in 1..word.len() + 1 {
                prefixes.push(word[0..i].into());
            }

            for pref in prefixes {
                let mut j_count = 0;
                if hmap.contains_key(&pref) {
                    i_count += hmap[&pref];
                } else {
                    for w in &words {
                        if pref.len() <= w.len() && pref == w[0..pref.len()] {
                            j_count += 1;
                        }
                    }
                    i_count += j_count;
                    hmap.insert(pref, j_count);
                }
            }

            output.push(i_count);
        }

        output
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
