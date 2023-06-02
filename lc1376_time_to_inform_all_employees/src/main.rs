// LeetCode 1376 https://leetcode.com/problems/time-needed-to-inform-all-employees/
// Martin Borgén

/* A company has n employees with a unique ID for each employee from 0 to n - 1. 
The head of the company is the one with headID.

Each employee has one direct manager given in the manager array 
where manager[i] is the direct manager of the i-th employee, manager[headID] = -1. 
Also, it is guaranteed that the subordination relationships have a tree structure.

The head of the company wants to inform all the company employees of an urgent piece of news. 
He will inform his direct subordinates, and they will inform their subordinates, 
and so on until all employees know about the urgent news.

The i-th employee needs informTime[i] minutes to inform all of his 
direct subordinates (i.e., After informTime[i] minutes, all his direct subordinates 
    an start spreading the news).

Return the number of minutes needed to inform all the employees about the urgent news.

Input: n = 6, headID = 2, manager = [2,2,-1,2,2,2], informTime = [0,0,1,0,0,0]
Output: 1
Explanation: The head of the company with id = 2 is the direct manager of all the employees 
in the company and needs 1 minute to inform them all.
The tree structure of the employees in the company is shown.

 */

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut largest = 0;

        for i in 0..n as usize{
            let mut time = 0;
            let mut man = manager[i];

            while man >= 0 {
                time += inform_time[man as usize];
                man = manager[man as usize];
            }
            if time > largest {
                largest = time;
            }
        }
        return largest;
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::num_of_minutes(1, 0, vec![-1], vec![0]), 0);
    assert_eq!(Solution::num_of_minutes(6, 2, vec![2,2,-1,2,2,2], vec![0,0,1,0,0,0]), 1);
    assert_eq!(Solution::num_of_minutes(10, 0, vec![-1, 0, 1, 2, 3, 4, 5, 6, 7, 8], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 0]), 36);
    assert_eq!(Solution::num_of_minutes(15, 0, vec![-1,0,0,1,1,2,2,3,3,4,4,5,5,6,6], vec![1,1,1,1,1,1,1,0,0,0,0,0,0,0,0]), 3);
}
