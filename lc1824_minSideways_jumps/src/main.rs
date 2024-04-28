impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let len = obstacles.len();

        let mut prev = vec![1, 0, 1];

        for pos in 1..len {
            let mut min_jumps = i32::MAX;
            for lane in 0..3_usize {
                if (lane + 1) as i32 == obstacles[pos] {
                    prev[lane] = i32::MAX;
                } else if prev[lane] < min_jumps {
                    min_jumps = prev[lane];
                }
            }

            for lane in 0..3_usize {
                if (lane + 1) as i32 == obstacles[pos] {
                    continue;
                }

                if prev[lane] > min_jumps {
                    prev[lane] = min_jumps + 1;
                }
            }
        }

        let mut out = i32::MAX;
        for lane in 0..3_usize {
            if prev[lane] < out {
                out = prev[lane];
            }
        }
        out
    }
}

struct Solution;

fn main() {
    assert_eq!(2, Solution::min_side_jumps(vec![0, 1, 2, 3, 0]));
    assert_eq!(0, Solution::min_side_jumps(vec![0, 1, 1, 3, 3, 0]));
    assert_eq!(2, Solution::min_side_jumps(vec![0, 2, 1, 0, 3, 0]));
}
