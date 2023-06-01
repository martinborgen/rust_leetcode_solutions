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
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;

struct Solution{}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // Idea is to use Boyer-Moore 
        let needle_vec: Vec<char> = needle.chars().collect();
        let haystack_vec: Vec<char> = haystack.chars().collect();
        let needle_len: usize = needle.chars().count();
        let haystack_len = haystack.chars().count();
        let mut shift: usize = 0;
        let mut i: usize = needle_len;

        // Hashing in the needle with it's indices. 
        // Each character in needle will have it's index stored in a vector. 
        // We loop through needle backwards to get the positions sorted from the rear
        let mut needle_hash: HashMap<char, Vec<usize>> = HashMap::new();
        for (i, c) in needle.chars().rev().enumerate() {
            match needle_hash.entry(c) {
                Entry::Vacant(e) => {e.insert(vec![i]);}
                Entry::Occupied(mut e) => {e.get_mut().push(i);} 
            }
        }

        loop {
            let haystack_char: char = haystack_vec[needle_len + shift];
            if haystack_char != needle_vec[needle_vec.len() -1] {
                if needle_hash.contains_key(&haystack_char) {
                    // exact expression here is still WIP
                    let next_i = needle_hash.get(&haystack_char).unwrap().first().unwrap();
                } else {
                    shift += needle_len;
                    if shift > haystack_len {
                        return -1;
                    }
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