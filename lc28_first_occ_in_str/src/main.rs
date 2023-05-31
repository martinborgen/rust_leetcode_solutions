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

    1 <= haystack.length, needle.length <= 104
    haystack and needle consist of only lowercase English characters.
*/
// use num_traits::cast::AsPrimitive;

struct Solution{}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // Idea is to use Boyer-Moore 
        let needle_it = needle.chars();
        let haystack_it = haystack.chars();
        let needle_len = needle.chars().count();
        let tst: i32 = i32::try_from(needle_len).unwrap();
        return tst;
    }
}

fn main() {
    let res = Solution::str_str(String::from("sadbutsad"), String::from("basd"));
    println!("{res}");
}