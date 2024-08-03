use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Task {
    index: i32,
    enqueue_t: i32,
    execute_t: i32,
}

// Explicit ordering to make binary heap a min-heap
impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.execute_t == other.execute_t {
            other.index.cmp(&self.index)
        } else {
            other.execute_t.cmp(&self.execute_t)
        }
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// impl PartialEq for Task {
//     fn eq(&self, other: &Self) -> bool {
//         self == other
//     }
// }

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tasks_sorted: Vec<Task> = Vec::with_capacity(tasks.len());
        let mut output: Vec<i32> = Vec::with_capacity(tasks.len());

        for (i, t) in tasks.iter().enumerate() {
            tasks_sorted.push(Task {
                index: i as i32,
                enqueue_t: t[0],
                execute_t: t[1],
            });
        }

        tasks_sorted.sort_by(|a, b| b.enqueue_t.cmp(&a.enqueue_t)); // reversing order

        let mut available_tasks: BinaryHeap<Task> = BinaryHeap::new();
        available_tasks.push(tasks_sorted.pop().unwrap());
        let mut time = available_tasks.peek().unwrap().enqueue_t;

        while !tasks_sorted.is_empty() || !available_tasks.is_empty() {
            // increment time in case no task is enqueued
            if !available_tasks.is_empty() && time < available_tasks.peek().unwrap().enqueue_t {
                time = available_tasks.peek().unwrap().enqueue_t;
            }

            // update available tasks
            while !tasks_sorted.is_empty() && tasks_sorted[tasks_sorted.len() - 1].enqueue_t <= time
            {
                available_tasks.push(tasks_sorted.pop().unwrap());
            }

            let current = available_tasks.pop().unwrap();
            time += current.execute_t;
            output.push(current.index);
        }
        output
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::get_order(vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]]),
        vec![0, 2, 3, 1]
    );

    assert_eq!(
        Solution::get_order(vec![
            vec![7, 10],
            vec![7, 12],
            vec![7, 5],
            vec![7, 4],
            vec![7, 2]
        ]),
        vec![4, 3, 2, 0, 1]
    );

    assert_eq!(
        Solution::get_order(vec![
            vec![19, 13],
            vec![16, 9],
            vec![21, 10],
            vec![32, 25],
            vec![37, 4],
            vec![49, 24],
            vec![2, 15],
            vec![38, 41],
            vec![37, 34],
            vec![33, 6],
            vec![45, 4],
            vec![18, 18],
            vec![46, 39],
            vec![12, 24]
        ]),
        vec![6, 1, 2, 9, 4, 10, 0, 11, 5, 13, 3, 8, 12, 7]
    );
}
