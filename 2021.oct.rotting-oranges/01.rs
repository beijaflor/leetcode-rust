// https://leetcode.com/submissions/detail/579067006/
use std::collections::VecDeque;

fn check_and_write(grid: &mut Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    match grid[row][col] {
        0 | 2=> { false },
        1 => {
            grid[row][col] = 2;
            true
        },
        _ => { panic!("invalid input!") }
    }
}

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut apples = 0;
        let mut turn = 0;
        let mut queue = VecDeque::<(usize, usize)>::new();
        (0..grid.len()).for_each(|row| {
            (0..grid[0].len()).for_each(|col| {
                match grid[row][col] {
                    0 => { /* NOP */ },
                    1 => { apples += 1; },
                    2 => { queue.push_back((row, col)); },
                    _ => { panic!("invalid input!") }
                }
            });
        });
        
        // println!("{:?}, {}", queue, apples);
        
        while apples != 0 && !queue.is_empty() {
            turn += 1;
            let mut next_queue = VecDeque::<(usize, usize)>::new();
            while let Some((row, col)) = queue.pop_front() {
                if col != 0 {
                    if check_and_write(&mut grid, row, col - 1) {
                        apples -= 1;
                        next_queue.push_back((row, col - 1));
                    }
                }

                if row != 0 {
                    if check_and_write(&mut grid, row - 1, col) {
                        apples -= 1;
                        next_queue.push_back((row - 1, col));
                    }
                }

                if col < grid[0].len() - 1 {
                    if check_and_write(&mut grid, row, col + 1) {
                        apples -= 1;
                        next_queue.push_back((row, col + 1));
                    }
                }

                if row < grid.len() - 1 {
                    if check_and_write(&mut grid, row + 1, col) {
                        apples -= 1;
                        next_queue.push_back((row + 1, col));
                    }
                }
            }
            std::mem::swap(&mut queue, &mut next_queue);
            
            // println!("{:?}", grid);
        }
        
        if apples != 0 {
            -1
        } else {
            turn
        }
    }
}