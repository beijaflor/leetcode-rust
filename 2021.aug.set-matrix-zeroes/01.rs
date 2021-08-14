// https://leetcode.com/submissions/detail/537968873/
use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut row: HashSet<usize> = HashSet::new();
        let mut col: HashSet<usize> = HashSet::new();

        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                if matrix[y][x] == 0 {
                    row.insert(x);
                    col.insert(y);
                }
            }
        }

        for y in (0..matrix.len()).rev() {
            for x in (0..matrix[y].len()).rev() {
                if row.contains(&x) || col.contains(&y) {
                    matrix[y][x] = 0;
                }
            }
        }
    }
}