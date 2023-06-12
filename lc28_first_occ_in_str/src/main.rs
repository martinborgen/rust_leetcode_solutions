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



impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // Idea is to use Boyer-Moore 
        let needle_vec: Vec<char> = needle.chars().collect();
        let haystack_vec: Vec<char> = haystack.chars().collect();
        let needle_len: usize = needle.chars().count();
        let haystack_len: usize = haystack.chars().count();
        
        // Hashing in the needle with it's indices. 
        // Each character in needle will have it's index stored in a vector. 
        let mut needle_hash: Vec<Vec<usize>> = vec![vec![]; 26];
        for (i, c) in needle.chars().enumerate() {
            needle_hash[c as usize - 97].push(i);
        }
        
        // Boyer-Moore loop
        let mut shift: usize = 0;           // shift is how far we've shifted needle along haystack
        
        'outer: loop {
            let mut i: usize = shift + needle_len - 1;      // i is the current index we're looking at when comparing needle and haystack
            
            if i > haystack_len - 1{
                return -1;
            }

            let nc_vec: &Vec<usize> =  &needle_hash[haystack_vec[i] as usize - 97]; // vector with all occurances(indexes) of nc in needle

            // following loop only fires if we have hc occuring in needle somewhere.
            for j in nc_vec.iter().rev() {
                if *j + shift == i {
                    // possible match, step backwards to see if they match
                    while needle_vec[i - shift] == haystack_vec[i] {
                        if i == shift {
                            // we've checked through needle and it all matched
                            return shift as i32;
                        } else {
                            i -= 1;
                        }
                    }
                    // if we're here, means comparision didn't match all the way
                    // => let 'inner continue; if we're out of chars in nc_vect, we drop out and let 'outer continue
                    i = shift + needle_len - 1;
                } else {
                    // means the index of j was wrong compared to i
                    // => we shift so j and i matches, then we let 'outer continue
                    shift += needle_len - 1 - *j;
                    continue 'outer;
                }
            }
            shift += needle_len;
        }
    }

}


struct Solution{}

fn main() {
    assert_eq!(Solution::str_str(String::from("sadbutsad"), String::from("sad")), 0);
    assert_eq!(Solution::str_str(String::from("sadbutsad"), String::from("but")), 3);
    assert_eq!(Solution::str_str(String::from("sadbutsad"), String::from("asd")), -1);
    assert_eq!(Solution::str_str(String::from("leetcode"), String::from("leeto")), -1);
    assert_eq!(Solution::str_str(String::from("hello"), String::from("ll")), 2);
    assert_eq!(Solution::str_str(String::from("aaaaa"), String::from("bba")), -1);
    

}