// https://leetcode.com/submissions/detail/486490022/
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 {
            return 0
        }

        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];
        
        dp[0][0] = 1;
        for index in 1..n {
            if obstacle_grid[0][index] == 0 {
                dp[0][index] = dp[0][index - 1];
            }
        }
        
        for y in (1..m) {
            for x in (0..n) {
                // println!("obstracle in {} / {} is {}", x, y, obstacle_grid[y][x]);
                if obstacle_grid[y][x] == 0 {
                    if x == 0 {
                        dp[y][x] = dp[y - 1][x];
                    } else {
                        dp[y][x] = dp[y - 1][x] + dp[y][x - 1];
                    }
                }
            }
        }
        
        // println!("{:?}", dp);
        dp[m - 1][n - 1]
    }
}