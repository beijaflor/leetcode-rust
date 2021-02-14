// https://leetcode.com/submissions/detail/455798488/
use std::collections::{VecDeque, BTreeSet};

impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut result = 1;
        let goal = (grid.len() - 1, grid[0].len() - 1);
        if grid[0][0] == 1 || grid[goal.0][goal.1] == 1 { return -1 }
        queue.push_back((0, 0));

        while !queue.is_empty() {
            let mut next: BTreeSet<(usize, usize)> = BTreeSet::new();
            while let Some((y, x)) = queue.pop_front() {
                // println!("{:?} at {}", (y, x), result);
                if (y, x) == goal { return result } 
                let y1 = if y == 0 { 0 } else { y - 1 };
                let y2 = if y == goal.0 { y } else { y + 1 };
                let x1 = if x == 0 { 0 } else { x - 1 };
                let x2 = if x == goal.1 { x } else { x + 1 };
                next.insert((y1, x));
                next.insert((y2, x));
                next.insert((y, x1));
                next.insert((y, x2));
                next.insert((y1, x1));
                next.insert((y2, x2));
                next.insert((y1, x2));
                next.insert((y2, x1));
            }
            // println!("next: {:?}", next);
            next.iter().for_each(|(y, x)| {
               if grid[*y][*x] == 0 {
                    queue.push_back((*y, *x));
               }
            });
            next.iter().for_each(|(y, x)| {
                grid[*y][*x] = 2;
            });
            // println!("{:?} at {}", queue, result);

            result += 1;
        }
        -1
    }
}