// https://leetcode.com/submissions/detail/479166342/
fn dfs(matrix: &Vec<Vec<i32>>, x: usize, y: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
    if cache[y][x] != 0 {
        cache[y][x]
    } else {
        if y > 0 {
            if matrix[y][x] > matrix[y - 1][x] {
                cache[y][x] = i32::max(cache[y][x], dfs(matrix, x, y - 1, cache));
            }
        }

        if x > 0 {
            if matrix[y][x] > matrix[y][x - 1] {
                cache[y][x] = i32::max(cache[y][x], dfs(matrix, x - 1, y, cache));
            }
        }

        if y < (matrix.len() - 1) {
            if matrix[y][x] > matrix[y + 1][x] {
                cache[y][x] = i32::max(cache[y][x], dfs(matrix, x, y + 1, cache));
            }
        }

        if x < (matrix[0].len() - 1) {
            if matrix[y][x] > matrix[y][x + 1] {
                cache[y][x] = i32::max(cache[y][x], dfs(matrix, x + 1, y, cache));
            }
        }

        cache[y][x] += 1;
        cache[y][x]
    }
}

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.len() == 0 { return 0 }
        let mut result = 0;
        let mut cache: Vec<Vec<i32>> = vec![vec![0; matrix[0].len()]; matrix.len()];

        for y in 0..(matrix.len()) {
            for x in 0..(matrix[0].len()) {
                result = i32::max(result, dfs(&matrix, x, y, &mut cache))
            }
        }
        
        result      
/*
// DFS + Memoization Solution
// Accepted and Recommended
public class Solution {
    private static final int[][] dirs = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};
    private int m, n;

    public int longestIncreasingPath(int[][] matrix) {
        if (matrix.length == 0) return 0;
        m = matrix.length; n = matrix[0].length;
        int[][] cache = new int[m][n];
        int ans = 0;
        for (int i = 0; i < m; ++i)
            for (int j = 0; j < n; ++j)
                ans = Math.max(ans, dfs(matrix, i, j, cache));
        return ans;
    }

    private int dfs(int[][] matrix, int i, int j, int[][] cache) {
        if (cache[i][j] != 0) return cache[i][j];
        for (int[] d : dirs) {
            int x = i + d[0], y = j + d[1];
            if (0 <= x && x < m && 0 <= y && y < n && matrix[x][y] > matrix[i][j])
                cache[i][j] = Math.max(cache[i][j], dfs(matrix, x, y, cache));
        }
        return ++cache[i][j];
    }
}
*/
    }
}