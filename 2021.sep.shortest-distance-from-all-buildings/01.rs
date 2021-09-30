// https://leetcode.com/submissions/detail/562370112/
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_distance(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut total: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
        let mut empty_value = 0;
        let mut min_dist = i32::MAX;
        (0..rows).for_each(|row| {
            (0..cols).for_each(|col| {
                if grid[row][col] == 1 {
                    min_dist = i32::MAX;

                    let mut queue = VecDeque::<(usize, usize)>::new();
                    let mut steps = 0;
                    queue.push_back((row, col));
                    
                    while !queue.is_empty() {
                        steps += 1;
                        
                        (0..queue.len()).rev().for_each(|level| {
                            let (row, col) = queue.pop_front().unwrap();
                            // println!("{} {} {}", row, col, empty_value);
                            
                            if row != 0 && grid[row - 1][col] == empty_value {
                                grid[row - 1][col] -= 1;
                                total[row - 1][col] += steps;
                                queue.push_back((row - 1, col));
                                min_dist = i32::min(min_dist, total[row - 1][col]);
                            }

                            if col != 0 && grid[row][col - 1] == empty_value {
                                grid[row][col - 1] -= 1;
                                total[row][col - 1] += steps;
                                queue.push_back((row, col - 1));
                                min_dist = i32::min(min_dist, total[row][col - 1]);
                            }

                            if row + 1 < rows && grid[row + 1][col] == empty_value {
                                grid[row + 1][col] -= 1;
                                total[row + 1][col] += steps;
                                queue.push_back((row + 1, col));
                                min_dist = i32::min(min_dist, total[row + 1][col]);
                            }

                            if col + 1 < cols && grid[row][col + 1] == empty_value {
                                grid[row][col + 1] -= 1;
                                total[row][col + 1] += steps;
                                queue.push_back((row, col + 1));
                                min_dist = i32::min(min_dist, total[row][col + 1]);
                            }
                        });
                    }
                    empty_value -= 1;
                }
            });
        });
        
        // println!("{:?}", min_dist);
        // println!("{:?}", total);
        // println!("{:?}", grid);
        
        if min_dist == i32::MAX { -1 } else { min_dist }
    }
}