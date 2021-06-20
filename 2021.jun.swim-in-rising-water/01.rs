// https://leetcode.com/submissions/detail/510573600/
use std::collections::BinaryHeap;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        let mut queue: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
        queue.push((-(grid[0][0]), 0, 0));
        
        while let Some((val, x, y)) = queue.pop() {
            // println!("{} at {} / {}", val, x, y);

            max = i32::max(max, -(val));

            if x == grid[0].len() - 1 && y == grid.len() - 1 {
                return max
            }

            if x > 0 {
                if visited[y][x - 1] == false {
                    visited[y][x - 1] = true;
                    queue.push((-(grid[y][x - 1]), x - 1, y));
                }
            }
            
            if y > 0 {
                if visited[y - 1][x] == false {
                    visited[y - 1][x] = true;
                    queue.push((-(grid[y - 1][x]), x, y - 1));
                }
            }

            if x < grid[0].len() - 1 {
                if visited[y][x + 1] == false {
                    visited[y][x + 1] = true;
                    queue.push((-(grid[y][x + 1]), x + 1, y));
                }
            }

            if y < grid.len() - 1 {
                if visited[y + 1][x] == false {
                    visited[y + 1][x] = true;
                    queue.push((-(grid[y + 1][x]), x, y + 1));
                }
            }
        }

        -1
    }
}