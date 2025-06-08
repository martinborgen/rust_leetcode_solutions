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

    1 <= num1 <= num2 <= 10^22
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
        let mut count = 0;

        // working_num is a number as a vector of it's digits. starts with num1, then increments
        let mut working_num: Vec<i32> = num1
            .chars()
            .map(|n| n.to_digit(10).unwrap() as i32)
            .collect();

        let num2_vect: Vec<i32> = num2
            .chars()
            .map(|n| n.to_digit(10).unwrap() as i32)
            .collect();

        let mut working_sum: i32 = working_num.iter().sum();
        loop {
            if min_sum <= working_sum && working_sum <= max_sum {
                if count == 1000000000 + 7 {
                    count = 1;
                } else {
                    count += 1;
                }
            }

            if working_num == num2_vect {
                break;
            }
            working_sum = Self::num_vect_incr(&mut working_num, working_sum);
        }

        count
    }

    /// increments the num vector and returns the sum of its digits
    fn num_vect_incr(num: &mut Vec<i32>, mut sum: i32) -> i32 {
        let mut i = num.len() - 1;
        while num[i] == 9 {
            num[i] = 0;
            sum -= 9;
            if i == 0 {
                num.push(0);
                // num[0] = 0; // this will be incremented later
            } else {
                i -= 1;
            }
        }
        num[i] += 1;
        sum += 1;
        sum
    }

    /// advances the numvect to minsum. takes the sum for efficiency reasons, returns it to ensure sucessful operation
    fn adv_to_minsum(num: &mut Vec<i32>, mut sum: i32, min_sum: i32, max_sum: i32) -> i32 {
        if num.len() == 0 {
            num.push(0);
        }

        let mut diff_min = min_sum - sum;
        let mut diff_max = max_sum - sum;

        if diff_max < 0 {
            // means we should flip a few digits to zeros
            let mut carry = 0;
            let mut i = num.len() - 1;

            // this loop only flips digits to zero (ten). It might not be necessary to flip all digits.
            while diff_min - carry < 0 && i > 0 {
                // num[i] increased so it becomes 10 and rolls over to 0
                diff_min += num[i];
                sum -= num[i];
                num[i] = 0;
                carry = 1; // we're flipping nums so carry is one if we flip. We use 'it up every flip' though, so never more than one
                i -= 1;

                // print!("num: ");
                // for x in num.iter() {
                //     print!("{x}");
                // }
                // print!("carry is {carry} \n");
            }

            if carry > 0 {
                // just add from current spot.
                while num[i] == 9 {
                    sum -= 9;
                    num[i] = 0;
                    if i == 0 {
                        num.push(0);
                    } else {
                        i -= 1;
                    }
                }
                num[i] += 1;
                sum += 1;
            }
        }

        let mut diff_min = min_sum - sum;
        if diff_min > 0 {
            let mut i = num.len() - 1;
            while i > 0 && diff_min > 0 {
                let incr = i32::min(9 - num[i], diff_min);
                diff_min -= incr;
                num[i] += incr;
                sum += incr;
                i -= 1;
            }

            if diff_min > 0 {
                // num[0] will be just nines in this case, right?
                while diff_min + num[i] > 9 {
                    num.push(9);
                    diff_min -= 9;
                    sum += 9;
                }

                num[i] += diff_min;
            }
        }
        sum
    }
}

fn main() {
    assert_eq!(Solution::count("1".into(), "12".into(), 1, 8), 11);
    assert_eq!(Solution::count("1".into(), "5".into(), 1, 5), 5);
    assert_eq!(Solution::count("14".into(), "234".into(), 13, 16), 41);
    assert_eq!(
        Solution::count("4179205230".into(), "7748704426".into(), 8, 46),
        883045899
    );
    assert_eq!(
        Solution::count("12345".into(), "22222222".into(), 17, 113),
        21740632
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_vect_incr() {
        let mut num_vect_test1 = vec![1, 2, 3];
        let sum1 = Solution::num_vect_incr(&mut num_vect_test1, 6);
        assert_eq!(num_vect_test1, [1, 2, 4]);
        assert_eq!(sum1, 7);

        let mut num_vect_test2 = vec![1, 8, 9, 9, 9];
        let sum2 = Solution::num_vect_incr(&mut num_vect_test2, 36);
        assert_eq!(num_vect_test2, [1, 9, 0, 0, 0]);
        assert_eq!(sum2, 10);

        let mut num_vect_test3 = vec![9];
        let sum3 = Solution::num_vect_incr(&mut num_vect_test3, 9);
        assert_eq!(num_vect_test3, [1, 0]);
        assert_eq!(sum3, 1);

        let mut num_vect_test4 = vec![9, 9, 9];
        let sum4 = Solution::num_vect_incr(&mut num_vect_test4, 27);
        assert_eq!(num_vect_test4, [1, 0, 0, 0]);
        assert_eq!(sum4, 1);
    }

    #[test]
    fn adv_to_minsum() {
        let minsum1 = 19; // min num for this is 199
        let maxsum1 = 24;
        let mut num_vec_test1 = vec![1, 2, 3];
        let sum1 = Solution::adv_to_minsum(&mut num_vec_test1, 6, minsum1, maxsum1);
        assert_eq!(sum1, minsum1);
        assert_eq!(num_vec_test1, [1, 9, 9]);

        let minsum2 = 19; // min num for this is 199
        let mut num_vec_test2 = vec![1];
        let sum2 = Solution::adv_to_minsum(&mut num_vec_test2, 1, minsum1, maxsum1);
        assert_eq!(sum2, minsum2);
        assert_eq!(num_vec_test2, [1, 9, 9]);

        let minsum3 = 19; // min num for this is 199
        let mut num_vec_test3 = vec![1, 2, 3];
        let sum3 = Solution::adv_to_minsum(&mut num_vec_test3, 6, minsum3, maxsum1);
        assert_eq!(sum3, minsum3);
        assert_eq!(num_vec_test3, [1, 9, 9]);

        let minsum4 = 19; // min num for this is 199
        let mut num_vec_test4 = vec![8, 9, 8]; // summa 25 dvs > maxsum1
        let sum4 = Solution::adv_to_minsum(&mut num_vec_test4, 25, minsum4, maxsum1);
        assert_eq!(sum4, minsum4);
        assert_eq!(num_vec_test4, [9, 1, 9]);

        let minsum5 = 19; // min num for this is 199
        let mut num_vec_test5 = vec![9, 9, 7];
        let sum5 = Solution::adv_to_minsum(&mut num_vec_test5, 25, minsum5, maxsum1);
        assert_eq!(sum5, minsum5);
        assert_eq!(num_vec_test5, [1, 0, 9, 9]);
    }
}
