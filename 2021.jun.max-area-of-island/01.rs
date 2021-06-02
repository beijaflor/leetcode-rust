// https://leetcode.com/submissions/detail/501614440/
use std::collections::VecDeque;

fn greedy(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut result = 0;
    queue.push_back((x, y));
    while let Some((x, y)) = queue.pop_front() {
        if grid[y][x] == 0 { continue }

        result += 1;
        grid[y][x] = 0;
        
        if x > 0 {
            queue.push_back((x - 1, y));
        }

        if y > 0 {
            queue.push_back((x, y - 1));
        }

        if x < grid[0].len() - 1 {
            queue.push_back((x + 1, y));
        }

        if y < grid.len() - 1 {
            queue.push_back((x, y + 1));
        }
    }
    result
}

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        (0..grid.len()).for_each(|y| {
            (0..grid[0].len()).for_each(|x| {
                // println!("at x: {}, y: {} = {}", x, y, grid[y][x]);
                if grid[y][x] == 1 {
                    result = i32::max(result, greedy(&mut grid, x, y));
                }
            });
        });
        result
    }
}