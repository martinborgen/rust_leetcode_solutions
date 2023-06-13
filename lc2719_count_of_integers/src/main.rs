// LeetCode problem 2719
// Martin Borgén
// 2023-06-04

impl Solution {
    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        let base: i32 = 10;
        let mod_fact = base.pow(9) + 7;

        // some obervations: the max digit sum for a 10**22 integer is 9*22 = 198.
        let n1: i128 = i128::from_str_radix(&num1, 10).unwrap();
        let n2: i128 = i128::from_str_radix(&num2, 10).unwrap();
        let mut count = 0;
        return count % (base.pow(9) + 7);
    }
    
    // num1 is start, num2 is stop. All vectors should have the same length 
    fn count_fun(num1: Vec<i32>, num2: Vec<i32>, min_sum: Vec<i32>, max_sum: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut old = 1;
        for i in 0..num2.len() {
            let digit_range = num2[i] - num1[i];
            count *= digit_range.abs();
        }
        return count;
    }

    fn digit_sum(num: i128) -> i32 {
        let mut sum: i128 = 0;
        let mut working = num;
        while working > 0 {
            sum += working % 10;
            working /= 10;
            // if sum > max as i128 {
            //     return -1;
            // }
        }
        return sum as i32;
    }

    fn digit_sum_str(num: &String, max: i32) -> String {
        let mut sum = 0;

        let mut max_digits = 0;
        let mut max_tmp = max;
        while max > 0 {
            max_tmp /= 10;
            max_digits += 1;
        }

        if num.len() as i32 * 9 < max {
            return num.clone();
        }

        for c in num.chars() {
            sum += c as i32 - 48;
            if sum >= max {
                return "-1".to_string();
            }
        }
        return sum;
    }
}

fn digit_sum(num: i32) -> i32 {
    let mut sum = 0;
    let mut working = num;
    while working > 0 {
        sum += working % 10;
        working /= 10;
    }
    return sum;
}

/// counts all possible digit sums form 1 to num, with sum within min and max
fn count(num1: i32, num2: i32, min: i32, max: i32) -> i32 {
    let mut count = 0;
    for i in num1..num2+1 {
        let res = digit_sum(i);
        if res >= min && res <= max {
            count += 1;
        }
    }
    return  count;
}

fn count2(num1: &mut Vec<i32>, num2: Vec<i32>, min: i32, max: i32) -> i32 {
    if num2.len() > num1.len() {
        num1.reverse();
        for _ in num1.len()..num2.len() {
            num1.push(0);
        }
        num1.reverse();
    }

    let mut out = 0;
    for i in 0..num2.len() {
        let diff  = num2[i] - num1[i];
        let base: i32 = 10;
        out += diff.abs() * base.pow((num2.len() as u32) - 1 - i as u32);
    }
    out += 1;
    return out;
}

fn count3(num1: Vec<i32>, num2: Vec<i32>, min: i32, max: i32) -> i32 {
    let mut working = num1.clone();
    if num2.len() > working.len() {
        working.reverse();
        for _ in working.len()..num2.len() {
            working.push(0);
        }
        working.reverse();
    }

    let old_sum = digisum_vect(&working);
    let count = 0;
    let mut i = working.len() - 1;
    while i > 0 {
        
        i -= 1;
    }

    return count
}

fn num_to_vect(num: i32) -> Vec<i32>{
    let mut outvect: Vec<i32> = Vec::new();
    let mut working = num;
    while working > 0 {
        outvect.push(working % 10);
        working /= 10;
    }
    outvect.reverse();
    return outvect;
}

fn digisum_vect(num: &Vec<i32>) -> i32 {
    let mut out = 0;
    for n in num {
        out += n;
    }
    return out;
}


struct Solution;

fn main() {
    let num1vect = vec![1,1,6,4,64,87,332];
    let num2vect = vec![12,5,23,32,132,124,863];
    let minsumvect = vec![1,1,8,4,4,7,1];
    let maxsumvect = vec![12,5,12,8,8,32,400];

    // TEST OF STRING DIGIT SUM 
    // assert_eq!(Solution::digit_sum_str(&"9999999999999999999999".to_string()),  198);
    // assert_eq!(Solution::digit_sum_str(&"10000000000000000000000".to_string()),  1);

    // TEST OF SOLUTION
    assert_eq!(Solution::count("1".to_string(), "12".to_string(), 1, 8), 11);
    assert_eq!(Solution::count("1".to_string(), "5".to_string(), 1, 5), 5);
    assert_eq!(Solution::count("1000000007".to_string(), "2000000014".to_string(), 1, 400), 1);
    assert_eq!(Solution::count("332".to_string(), "863".to_string(), 20, 29), 61);
    assert_eq!(Solution::count("6".to_string(), "23".to_string(), 8, 12), 5);
    assert_eq!(Solution::count("4".to_string(), "32".to_string(), 4, 8), 17);
    // assert_eq!(Solution::count("862662375825717340797".to_string(), "1341489267173864014004".to_string(), 204, 338), 0);
    // assert_eq!(Solution::count("1".to_string(), "10000000000000000000000".to_string(), 1, 400), 1);
}   
