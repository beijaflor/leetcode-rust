// https://leetcode.com/submissions/detail/588434131/
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut map = vec![vec![0; n]; m];
        
        (0..n).for_each(|col| {
            (0..m).for_each(|row| {
                if col == 0 || row == 0 {
                    map[row][col] = 1;
                } else {
                    map[row][col] = map[row - 1][col] + map[row][col - 1];
                }
            });
        });
        
        map[m - 1][n - 1]
    }
}