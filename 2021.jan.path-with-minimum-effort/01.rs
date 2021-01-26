// https://leetcode.com/submissions/detail/448109678/
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let m = heights.len();
        let n = heights[0].len();
        let mut diff_mat: Vec<Vec<i32>> = Vec::new();
        let mut visited_mat: Vec<Vec<bool>> = Vec::new();
        for _ in 0..m {
            let mut diff_row = Vec::with_capacity(n);
            let mut visited_row = Vec::with_capacity(n);
            for _ in 0..n {
                diff_row.push(std::i32::MAX);                
                visited_row.push(false);                
            }
            diff_mat.push(diff_row);
            visited_mat.push(visited_row);
        }

        diff_mat[0][0] = 0;

        let mut queue: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
        queue.push((0, 0, 0));

        while !queue.is_empty() {
            let q = queue.pop().unwrap();
            // println!("current: {:?}", q);
            let current = heights[q.1][q.2];
            let pos: (i32, usize, usize) = (-q.0, q.1, q.2);
            visited_mat[pos.1][pos.2] = true;
            if pos.1 == n - 1 && pos.2 == m - 1 {
                break
            }

            for num in 0..4 {
                let new_pos: (i32, usize, usize);
    
                if num == 0 && !(pos.1 == 0) {
                    new_pos = (0, pos.1 - 1, pos.2)
                } else
    
                if num == 1 && !(pos.2 == 0) {
                    new_pos = (0, pos.1, pos.2 - 1)
                } else
    
                if num == 2 && pos.1 + 1 < m {
                    new_pos = (0, pos.1 + 1, pos.2)
                } else
    
                if num == 3 && pos.2 + 1 < n {
                    new_pos = (0, pos.1, pos.2 + 1)
                } else {
                    continue;
                }
    
                if !visited_mat[new_pos.1][new_pos.2] {
                    let diff = (current - heights[new_pos.1][new_pos.2]).abs();
                    let max = i32::max(diff, pos.0);
                    if max < diff_mat[new_pos.1][new_pos.2] {
                        diff_mat[new_pos.1][new_pos.2] = max;
                        queue.push((-max, new_pos.1, new_pos.2));
                    }
                }
            }
            // println!("{:?}", queue);
        }

        // println!("{:?}", diff_mat);

        diff_mat[m - 1][n - 1]
    }
}