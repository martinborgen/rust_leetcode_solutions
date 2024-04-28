impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let len = obstacles.len();

        // let visited: Vec<Vec<i32>> = vec![vec![0; 3]; len];
        let mut visited: Vec<Vec<i32>> = vec![vec![i32::MAX; 3]; len];

        visited[0][0] = 1;
        visited[0][1] = 0;
        visited[0][2] = 1;

        for pos in 0..visited.len() {
            for lane in 0..3 as usize{
                if (lane + 1) as i32 == obstacles[pos] {
                    continue;
                }

                if obstacles[pos - 1] != (lane + 1) as i32 {
                    visited[pos][lane] = visited[pos - 1][lane];
                }
            }

            let mut min_jumps = i32::MAX;
            for lane in 0..3 as usize {
                if visited[pos][lane] < min_jumps {
                    min_jumps = visited[pos][lane];
                }
            }

            for lane in 0..3 as usize {
                if (lane + 1) as i32 == obstacles[pos] {
                    continue;
                }

                if visited[pos][lane] > min_jumps {
                    visited[pos][lane] = min_jumps + 1;
                }
            }
        }

        let mut out = i32::MAX;
        for lane in 0..3 as usize {
            if visited[len - 1][lane] < out {
                out = visited[len - 1][lane];
            }
        }
        return out;
    }
}

struct Solution;

fn main() {
    Solution::min_side_jumps(vec![0, 1, 2, 3, 0]);
}
