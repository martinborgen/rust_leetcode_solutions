// You are given a positive integer k. You are also given:

//     a 2D integer array rowConditions of size n where rowConditions[i] = [abovei, belowi], and
//     a 2D integer array colConditions of size m where colConditions[i] = [lefti, righti].

// The two arrays contain integers from 1 to k.

// You have to build a k x k matrix that contains each of the numbers from 1 to k exactly once. The remaining cells should have the value 0.

// The matrix should also satisfy the following conditions:

//     The number abovei should appear in a row that is strictly above the row at which the number belowi appears for all i from 0 to n - 1.
//     The number lefti should appear in a column that is strictly left of the column at which the number righti appears for all i from 0 to m - 1.

// Return any matrix that satisfies the conditions. If no answer exists, return an empty matrix.

impl Solution {
    pub fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut out: Vec<Vec<i32>> = vec![vec![0; k as usize]; k as usize];
        
        let mut k_and_pos: Vec<Vec<i32>> = vec![vec![0; 3]; k as usize]; // is [k, row, col]
        for i in 0..k {
            k_and_pos[i as usize] = vec![i + 1; 3];
        }
        
        print_matrix(k_and_pos);
        todo!()
    }
}

struct Solution;

fn print_matrix(matrix: Vec<Vec<i32>>) {
    for m in matrix {
        for n in m {
            print!("{} ", n);
        }
        print!("\n");
    }
}

fn main() {
    Solution::build_matrix(6, vec![vec![0, 0]], vec![vec![0, 0]]);
}
