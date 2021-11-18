// https://leetcode.com/submissions/detail/580717886/
struct Solver {
    grid: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
    path_count: i32,
}

impl Solver {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let rows = grid.len();
        let cols = grid[0].len();
        Solver {
            grid, rows, cols,
            path_count: 0,
        }
    }
    
    fn backtrack(&mut self, row: usize, col: usize, remain: i32) {
        // println!("{:?}", remain);
        // println!("{:?}", self.grid);
        if self.grid[row][col] == 2 && remain == 1 {
            // println!("GOAL!!!");
            self.path_count += 1;
            return;
        }
        
        let temp = self.grid[row][col];
        self.grid[row][col] = -4;
        let remain = remain - 1;
        
        if row != 0 && self.grid[row - 1][col] > -1 {
            self.backtrack(row - 1, col, remain);
        }

        if col != 0 && self.grid[row][col - 1] > -1 {
            self.backtrack(row, col - 1, remain);
        }

        if row < self.grid.len() - 1 && self.grid[row + 1][col] > -1 {
            self.backtrack(row + 1, col, remain);
        }

        if col < self.grid[0].len() - 1 && self.grid[row][col + 1] > -1 {
            self.backtrack(row, col + 1, remain);
        }

        
        self.grid[row][col] = temp;
    }
}

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut start_row = 0;
        let mut start_col = 0;
        let non_obstacles = (0..grid.len()).fold(0, |acc, row| {
            acc + (0..grid[0].len()).fold(0, |acc, col| {
                match grid[row][col] {
                    0 | 2 => { acc + 1 },
                    1 => {
                        start_row = row;
                        start_col = col;
                        acc + 1
                    },
                    _ => { acc },
                }
            })
        });
        
        // println!("{} {} {}", start_row, start_col, non_obstacles);
        
        let mut solver = Solver::new(grid);
        solver.backtrack(start_row, start_col, non_obstacles);

        solver.path_count
    }
}