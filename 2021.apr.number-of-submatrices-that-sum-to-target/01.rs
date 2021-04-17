// https://leetcode.com/submissions/detail/481701532/
use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let r = matrix.len();
        let c = matrix[0].len();
        
        let mut prefix_sum = vec![vec![0; c + 1]; r + 1];
        for i in 1..(r+1) {
            for j in 1..(c+1) {
                prefix_sum[i][j] = prefix_sum[i - 1][j] + prefix_sum[i][j - 1] - prefix_sum[i - 1][j - 1] + matrix[i - 1][j - 1];
            }
        }
        
        // println!("{:?}", prefix_sum);
        
        let mut count = 0;
        
        for r1 in 1..(r+1) {
            for r2 in r1..(r+1) {

                let mut map: HashMap<i32, i32> = HashMap::new();
                let mut current_sum = 0;
                map.insert(0, 1);

                for col in 1..(c+1) {
                    current_sum = prefix_sum[r2][col] - prefix_sum[r1 - 1][col];
                    count += match map.get(&(current_sum - target)) { None => 0, Some(val) => *val};
                    map.insert(current_sum, match map.get(&current_sum) { None => 1, Some(val) => *val + 1});
                }
                // println!("{:?}", map);
                // println!("count: {:?}", count);
            }
        }
        
        count
    }
}
/*
class Solution {
  public int numSubmatrixSumTarget(int[][] matrix, int target) {
    int r = matrix.length, c = matrix[0].length;

    // compute 2D prefix sum
    int[][] ps = new int[r + 1][c + 1];
    for (int i = 1; i < r + 1; ++i) {
      for (int j = 1; j < c + 1; ++j) {
        ps[i][j] = ps[i - 1][j] + ps[i][j - 1] - ps[i - 1][j - 1] + matrix[i - 1][j - 1];
      }
    }

    int count = 0, currSum;
    Map<Integer, Integer> h = new HashMap();
    // reduce 2D problem to 1D one
    // by fixing two rows r1 and r2 and 
    // computing 1D prefix sum for all matrices using [r1..r2] rows
    for (int r1 = 1; r1 < r + 1; ++r1) {
      for (int r2 = r1; r2 < r + 1; ++r2) {
        h.clear();
        h.put(0, 1);
        for (int col = 1; col < c + 1; ++col) {
          // current 1D prefix sum
          currSum = ps[r2][col] - ps[r1 - 1][col];

          // add subarrays which sum up to (currSum - target)
          count += h.getOrDefault(currSum - target, 0);

          // save current prefix sum
          h.put(currSum, h.getOrDefault(currSum, 0) + 1);
        }
      }
    }

    return count;
  }
}
*/