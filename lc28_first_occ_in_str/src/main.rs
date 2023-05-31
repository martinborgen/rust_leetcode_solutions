/*  Leetcode problem 28
    Martin Borgén
    2023-05-31

 Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

 

Example 1:

Input: haystack = "sadbutsad", needle = "sad"
Output: 0
Explanation: "sad" occurs at index 0 and 6.
The first occurrence is at index 0, so we return 0.

Example 2:

Input: haystack = "leetcode", needle = "leeto"
Output: -1
Explanation: "leeto" did not occur in "leetcode", so we return -1.

 

Constraints:

    1 <= haystack.length, needle.length <= 10^4
    haystack and needle consist of only lowercase English characters.
*/
// use num_traits::cast::AsPrimitive;
// use std::collections::HashMap;
use std::collections::HashSet;

struct Solution{}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // Idea is to use Boyer-Moore 
        let needle_vec: Vec<char> = needle.chars().collect();
        let haystack_vec: Vec<char> = haystack.chars().collect();
        let needle_len: usize = needle.chars().count();
        let shift: usize = 0;
        let i: usize = 0;

        // Hashing in the needle with it's indices- SKIP FOR NOW; COLLISIONS MAKES IT HARDER
        // let mut needle_hash: HashMap<usize, char> = HashMap::new();

        let mut needle_hash = HashSet::new();
        for c in needle.chars() {
            needle_hash.insert(c);
        }

        while true {
            let haystack_char: char = haystack_vec[needle_len];
            if haystack_char != needle_vec[needle_vec.len() -1] {
                if needle_hash.contains(&haystack_char) {
                    
                }
            }
            
        }

        return 0;
    }

}

fn main() {
    let res = Solution::str_str(String::from("sadbutsad"), String::from("basd"));
    println!("{res}");
}