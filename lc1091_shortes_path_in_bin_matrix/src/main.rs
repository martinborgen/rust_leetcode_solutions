// Leetcode 1091 shortest path 
// Martin Borgén
// 2023-06-01

use std::cmp::min;
use std::collections::{VecDeque, HashSet};
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

#[derive(Eq)]
struct Cell {
    pos: (usize, usize),
    dist: i32,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();  // The length of the matrix side
        if grid[0][0] == 1 || grid[n-1][n-1] == 1 { // guard clause against edge-case
            return -1;
        }

        // Implements a BFS search
        let mut cell_q = VecDeque::new();
        let mut visited = HashSet::new();
        cell_q.push_back(Cell{pos:(0, 0), dist:1});

        while !cell_q.is_empty() {
            let current = cell_q.pop_front().unwrap();
            if current.pos.0 == n-1 && current.pos.1 == n-1 {
                return current.dist;
            }
            
            let adjs = Self::get_adjacents(&grid, &current, &visited);
            visited.insert(current);
            for adj in adjs {
                if !visited.contains(&adj) {
                    cell_q.push_back(adj);
                }
            }
        }
        return -1;
    }

    /// Returns the adjecant cells.
    fn get_adjacents(grid: &Vec<Vec<i32>>, origin: &Cell, visited: &HashSet<Cell>) -> Vec<Cell> {
        let mut out: Vec<Cell> = Vec::new();
        let n = grid[0].len() - 1; // length of matrix side, adjusted to match 0-indexing

        let r0 = origin.pos.0 - 1 * usize::try_from(origin.pos.0 > 0).unwrap();
        let r1 = min(origin.pos.0 + 1, n);

        let c0 = origin.pos.1 - 1 * usize::try_from(origin.pos.1 > 0).unwrap();
        let c1 = min(origin.pos.1 + 1, n);

        for r in r0..r1+1 {
            for c in c0..c1+1 {
                let adj = grid[r][c];
                let pot = Cell{pos: (r, c), dist: origin.dist+1};
                if adj == 0 && !visited.contains(&pot) && pot != *origin{
                    out.push(pot);
                }
            }
        }
        out
    }
}

fn main() {
    assert_eq!(Solution::shortest_path_binary_matrix(vec![vec![0,0,0], 
                                                          vec![1,1,0], 
                                                          vec![1,1,0]]), 4);
    
    assert_eq!(Solution::shortest_path_binary_matrix(vec![vec![1,0,0], 
                                                          vec![1,1,0], 
                                                          vec![1,1,0]]), -1);
    
    assert_eq!(Solution::shortest_path_binary_matrix(vec![vec![0,1,1], 
                                                          vec![1,0,1], 
                                                          vec![1,1,0]]), 3);
    
    assert_eq!(Solution::shortest_path_binary_matrix(vec![vec![0,1,1], 
                                                          vec![0,1,1], 
                                                          vec![0,0,0]]), 4);
    
    assert_eq!(Solution::shortest_path_binary_matrix(vec![vec![0,0,0,0,0], 
                                                          vec![1,0,1,1,0], 
                                                          vec![1,0,1,1,0], 
                                                          vec![1,1,0,1,0],
                                                          vec![1,1,0,0,0]]), 6);

    assert_eq!(Solution::shortest_path_binary_matrix(vec![vec![0,1,1,0,0], 
                                                          vec![1,0,1,1,0], 
                                                          vec![1,0,1,1,0], 
                                                          vec![1,0,1,1,0],
                                                          vec![1,0,0,0,0]]), 7);

    assert_eq!(Solution::shortest_path_binary_matrix(vec![vec![0,1,1,1,1,1,1,1],
                                                          vec![0,1,1,0,0,0,0,0],
                                                          vec![0,1,0,1,1,1,1,0],
                                                          vec![0,1,0,1,1,1,1,0],
                                                          vec![0,1,1,0,0,1,1,0],
                                                          vec![0,1,1,1,1,0,1,0],
                                                          vec![0,0,0,0,0,1,1,0],
                                                          vec![1,1,1,1,1,1,1,0]]), 25);                                   

}
