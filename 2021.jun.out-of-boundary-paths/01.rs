// https://leetcode.com/submissions/detail/512799797/
impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let modulo = 1_000_000_007 as i64;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];
        dp[start_row as usize][start_column as usize] = 1;
        let mut result = 0;
        
        // println!("{:?}", dp);

        (1..=max_move).for_each(|_| {
            let mut tmp: Vec<Vec<i32>> = vec![vec![0; n]; m];
            // println!("{:?}", dp);
            (0..m).for_each(|row| {
                (0..n).for_each(|col| {
                    // println!("{}, {} / {}, {}", row, col, m, n);
                    let mut bound: i64 = 0;
                    if row == m - 1 {bound += 1 } else {
                        tmp[row][col] = ((tmp[row][col] + dp[row + 1][col]) as i64 % modulo) as i32;
                    }
                    if col == n - 1 { bound += 1 } else {
                        tmp[row][col] = ((tmp[row][col] + dp[row][col + 1]) as i64 % modulo) as i32;
                    }
                    if row == 0 { bound += 1 } else {
                        tmp[row][col] = ((tmp[row][col] + dp[row - 1][col]) as i64 % modulo) as i32;
                    }
                    if col == 0 { bound += 1 } else {
                        tmp[row][col] = ((tmp[row][col] + dp[row][col - 1]) as i64 % modulo) as i32;
                    }
                    result = ((result as i64 + dp[row][col] as i64 * bound) % modulo) as i32;
                });
            });
            dp = tmp;
        });
        
        result
    }
}