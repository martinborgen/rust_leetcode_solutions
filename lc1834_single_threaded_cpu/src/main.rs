struct Task {
    index: i32,
    enqueue_t: i32,
    execute_t: i32,
}

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

        tasks_sorted.sort_by(|a, b| {
            if a.enqueue_t == b.enqueue_t {
                a.enqueue_t.cmp(&b.execute_t)
            } else {
                a.enqueue_t.cmp(&b.enqueue_t)
            }
        });

        tasks_sorted.reverse(); // reverse because Vec::pop() returns last element

        let mut time = tasks_sorted[tasks_sorted.len() - 1].enqueue_t;
        let mut available_tasks: Vec<Task> = Vec::new();
        loop {
            while tasks_sorted.len() > 0 && tasks_sorted[tasks_sorted.len() - 1].enqueue_t <= time {
                available_tasks.push(tasks_sorted.pop().unwrap())
            }

            let mut min_idx: usize = 0;
            let mut min = i32::MAX;
            for (i, t) in available_tasks.iter().enumerate() {
                if t.execute_t < min {
                    min_idx = i;
                    min = t.execute_t;
                }
            }

            let current_task = available_tasks.remove(min_idx);
            time += current_task.execute_t;
            output.push(current_task.index);

            if output.len() == tasks.len() {
                break;
            }
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
}
