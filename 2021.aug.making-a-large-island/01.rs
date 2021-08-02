// https://leetcode.com/submissions/detail/531644197/
use std::collections::{HashSet, HashMap, VecDeque};

fn set_size(grid: &mut Vec<Vec<i32>>, x: usize, y: usize, id: i32) -> Option<i32> {
    if grid[y][x] != 1 { return None }
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((x, y));
    while let Some((x, y)) = queue.pop_front() {
        set.insert((x, y));
        if x != 0 && grid[y][x - 1] == 1 {
            if !set.contains(&(x - 1, y)) && !queue.contains(&(x - 1, y)) {
                queue.push_back((x - 1, y));
            }
        }

        if y != 0 && grid[y - 1][x] == 1 {
            if !set.contains(&(x, y - 1)) && !queue.contains(&(x, y - 1)) {
                queue.push_back((x, y - 1));
            }
        }

        if x != grid[0].len() - 1 && grid[y][x + 1] == 1 {
            if !set.contains(&(x + 1, y)) && !queue.contains(&(x + 1, y)) {
                queue.push_back((x + 1, y));
            }
        }

        if y != grid.len() - 1 && grid[y + 1][x] == 1 {
            if !set.contains(&(x, y + 1)) && !queue.contains(&(x, y + 1)) {
                queue.push_back((x, y + 1));
            }
        }
    }
    
    let size = set.len() as i32;
    set.into_iter().for_each(|(x, y)| grid[y][x] = id);
    Some(size)
}

impl Solution {
    pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut id = i32::MAX;
        let mut size_map: HashMap<i32, i32> = HashMap::new();
        (0..grid.len()).for_each(|y| {
            (0..grid[0].len()).for_each(|x| {
                if let Some(size) = set_size(&mut grid, x, y, id) {
                    size_map.insert(id, size);
                    id -= 1;
                };
            });
        });
        // println!("{:?}", grid);
        // println!("{:?}", size_map);
        
        let mut max = 0;
        (0..grid.len()).for_each(|y| {
            (0..grid[0].len()).for_each(|x| {
                let mut around: HashSet<i32> = HashSet::new();
                if grid[y][x] == 0 {
                    if x != 0 {
                        around.insert(grid[y][x - 1]);
                    }

                    if y != 0 {
                        around.insert(grid[y - 1][x]);
                    }

                    if x != grid[0].len() - 1 {
                        around.insert(grid[y][x + 1]);
                    }

                    if y != grid.len() - 1 {
                        around.insert(grid[y + 1][x]);
                    }
                    
                    let mut size_vec: Vec<i32> = around.into_iter().map(|id| *size_map.get(&id).unwrap_or(&0)).collect();
                    let size = size_vec.into_iter().fold(1, |acc, size| {
                        acc + size
                    });
                    max = i32::max(max, size);
                } else {
                    max = i32::max(max, *size_map.get(&grid[y][x]).unwrap());
                }
            });
        });
        
        max
    }
}