// https://leetcode.com/submissions/detail/552216445/
fn collect_rows(grid: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    (0..grid.len()).for_each(|r| {
        (0..grid[0].len()).for_each(|c| {
            if grid[r][c] == 1 {
                result.push(r as i32);
            }
        });
    });
    result
}

fn collect_cols(grid: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    (0..grid[0].len()).for_each(|c| {
        (0..grid.len()).for_each(|r| {
            if grid[r][c] == 1 {
                result.push(c as i32);
            }
        });
    });
    result
}

fn min_distance_1d(points: Vec<i32>) -> i32 {
    let mut distance = 0;
    let mut l = 0;
    let mut r = points.len() - 1;
    while l < r {
        distance += points.get(r).unwrap() - points.get(l).unwrap();
        l += 1;
        r -= 1;
    }
    distance
}

impl Solution {
    pub fn min_total_distance(grid: Vec<Vec<i32>>) -> i32 {
        let rows: Vec<i32> = collect_rows(&grid);
        let cols: Vec<i32> = collect_cols(&grid);
        
        min_distance_1d(rows) + min_distance_1d(cols)
    }
}

