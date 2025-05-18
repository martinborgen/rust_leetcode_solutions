/* You are given two numeric strings num1 and num2 and two integers max_sum and min_sum. We denote an integer x to be good if:

    num1 <= x <= num2
    min_sum <= digit_sum(x) <= max_sum.

Return the number of good integers. Since the answer may be large, return it modulo 109 + 7.

Note that digit_sum(x) denotes the sum of the digits of x.



Example 1:

Input: num1 = "1", num2 = "12", min_sum = 1, max_sum = 8
Output: 11
Explanation: There are 11 integers whose sum of digits lies between 1 and 8 are 1,2,3,4,5,6,7,8,10,11, and 12. Thus, we return 11.

Example 2:

Input: num1 = "1", num2 = "5", min_sum = 1, max_sum = 5
Output: 5
Explanation: The 5 integers whose sum of digits lies between 1 and 5 are 1,2,3,4, and 5. Thus, we return 5.



Constraints:

    1 <= num1 <= num2 <= 1022
    1 <= min_sum <= max_sum <= 400

 */

struct Solution;

/*
Some notes and ideas:
if incrementing from a number with a digit sum less than the min sum, increment
directly until min sum is reached.

when reaching the max sum, stop incrementing and go to the next n+1 digit number
somewhere, permutations should also be possible to add, since as long as a number
is within the range of [num1, num2], but it might be hard to account for double-
counting numbers that way.
*/

impl Solution {
    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        let mut count: i128 = 0;

        let base = 10;
        let n1 = i128::from_str_radix(&num1, base).unwrap(); // intentional panic on parseIntError
        let n2 = i128::from_str_radix(&num2, base).unwrap(); // intentional panic on parseIntError

        let mut i = n1;

        while i < n2 {
            let i_sum = Self::digit_sum_int(i);
            if i_sum < min_sum {}
        }

        (count % (1000000000 + 7)) as i32
    }

    fn num_vect_incr(num: &mut Vec<i32>) {
        let mut i = num.len() - 1;
        while num[i] == 9 {
            num[i] = 0;

            if i == 0 {
                num.push(0);
                for j in (1..num.len()).rev() {
                    num[j] = num[j - 1];
                }
                num[0] = 0; // this will be incremented later
            } else {
                i -= 1;
            }
        }

        num[i] += 1;
    }

    fn print_numvect(num: &Vec<i32>) {
        for n in num.iter() {
            print!("{}", n);
        }
    }

    fn digit_sum_int(mut num: i128) -> i32 {
        let mut sum = 0;
        while num > 0 {
            sum += (num % 10) as i32;
            num /= 10;
        }
        sum
    }
}

fn main() {
    // let mut num_vect_test1 = vec![1, 2, 3];
    // Solution::num_vect_incr(&mut num_vect_test1);
    // assert_eq!(num_vect_test1, [1, 2, 4]);

    // let mut num_vect_test2 = vec![1, 8, 9, 9, 9];
    // Solution::num_vect_incr(&mut num_vect_test2);
    // assert_eq!(num_vect_test2, [1, 9, 0, 0, 0]);

    // let mut num_vect_test3 = vec![9];
    // Solution::num_vect_incr(&mut num_vect_test3);
    // assert_eq!(num_vect_test3, [1, 0]);

    // let mut num_vect_test4 = vec![9, 9, 9];
    // Solution::num_vect_incr(&mut num_vect_test4);
    // assert_eq!(num_vect_test4, [1, 0, 0, 0]);

    // assert_eq!(Solution::count("1".into(), "12".into(), 1, 8), 11);
    // assert_eq!(Solution::count("1".into(), "5".into(), 1, 5), 5);
    // assert_eq!(
    //     Solution::count("4179205230".into(), "7748704426".into(), 8, 46),
    //     883045899
    // );
    // assert_eq!(
    //     Solution::count("12345".into(), "22222222".into(), 17, 113),
    //     21740632
    // )

    assert_eq!(Solution::count("45".into(), "222".into(), 12, 113), 9);
}
