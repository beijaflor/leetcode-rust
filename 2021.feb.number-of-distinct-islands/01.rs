// https://leetcode.com/submissions/detail/455416336/
use std::collections::{BTreeSet, VecDeque};

fn find_island(y: usize, x: usize, grid: &mut Vec<Vec<i32>>) -> (usize, Vec<usize>) {
    let mut island: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((y, x));
    // println!("{:?}, {}", queue, grid[y][x]);
    while !queue.is_empty() {
        let (y, x) = queue.pop_back().unwrap();
        // println!("{}, {} {}", x, y, grid[y][x]);
        if grid[y][x] == 1 {
            grid[y][x] = 0;
            island.insert((x, y));
            queue.push_back((y - 1, x));
            queue.push_back((y + 1, x));
            queue.push_back((y, x - 1));
            queue.push_back((y, x + 1));
        }
    }
    // println!("{:?}", island);

    let mut iter = island.iter();
    let (mut start_y, mut start_x) = *iter.next().unwrap();
    let mut end_y = start_y;
    let mut end_x = start_x;
    island.iter().for_each(|(y, x)| {
        start_y = usize::min(start_y, *y);        
        start_x = usize::min(start_x, *x);        
        end_y = usize::max(end_y, *y);        
        end_x = usize::max(end_x, *x);        
    });

    let mut result: Vec<Vec<usize>> = vec![];
    for _ in start_y..end_y + 1 {
        let mut row = vec![];
        for _ in start_x..end_x + 1 {
            row.push(0);
        }
        result.push(row);
    }

    // println!("{:?}", result);

    island.iter().for_each(|(y, x)| {
       result[y - start_y][x - start_x] = 1; 
    });

    // println!("{:?}", result);

    (end_y - start_y + 1, result.concat())
}

impl Solution {
    pub fn num_distinct_islands(grid: Vec<Vec<i32>>) -> i32 {
        // prepare: add canal around grid
        let mut grid: Vec<Vec<i32>> = grid.into_iter().map(|row| {
            [vec![0], row, vec![0]].concat()
        }).collect();
        let len = grid[0].len();
        let mut line: Vec<i32> = Vec::with_capacity(len);
        for _ in 0..len {
            line.push(0);
        }
        let line2 = line.clone();
        grid.insert(0, line);
        grid.push(line2);

        // println!("{:?}", grid);

        // main
        let mut result: BTreeSet<(usize, Vec<usize>)> = BTreeSet::new();
        let mut x_pointer = 1;
        let mut y_pointer = 1;
        while x_pointer < grid[0].len() && y_pointer < grid.len() {
            // println!("x: {}, y: {}", x_pointer, y_pointer);
            if grid[y_pointer][x_pointer] == 1 {
                result.insert(find_island(y_pointer, x_pointer, &mut grid));
            }
            x_pointer += 1;
            if x_pointer == grid[0].len() {
                x_pointer = 0;
                y_pointer += 1;
            }
        }
        // println!("{:?}", result);
        result.len() as i32
    }
}