// https://leetcode.com/submissions/detail/565448864/
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        (0..grid.len()).for_each(|row| {
            (0..grid[0].len()).for_each(|col| {
                if grid[row][col] != 1 {
                    return
                }
                if col == 0 || grid[row][col - 1] == 0 {
                    result += 1;
                }
                if col == grid[0].len() - 1 || grid[row][col + 1] == 0 {
                    result += 1;
                }
                if row == 0 || grid[row - 1][col] == 0 {
                    result += 1;
                }
                if row == grid.len() - 1 || grid[row + 1][col] == 0 {
                    result += 1;
                }
            });
        });
        
        result
    }
}