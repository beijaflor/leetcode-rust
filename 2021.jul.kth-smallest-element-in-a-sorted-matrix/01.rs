// https://leetcode.com/submissions/detail/518747760/
fn count(matrix: &Vec<Vec<i32>>, mid: i32, mut small: i32, mut large: i32) -> (i32, i32, i32) {
    let mut count: i32 = 0;
    let n = matrix.len();
    let mut row = n - 1;
    let mut col = 0;
    
    while col < n {
        // println!("{}/ {}", row, col);
        if matrix[row][col] > mid {
            large = i32::min(large, matrix[row][col]);
            if row == 0 { break }
            row -= 1;
        } else {
            small = i32::max(small, matrix[row][col]);
            count += row as i32 + 1;
            col += 1;
        }
    }
    println!("\n");
    (count, small, large)
}

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let mut start = matrix[0][0];
        let mut end = matrix[n - 1][n - 1];
        while start < end {
            let mut mid = start + (end - start) / 2;
            let mut small = matrix[0][0];
            let mut large = matrix[n -1][n - 1];
            
            let (count, small, large) = count(&matrix, mid, small, large);
            
            if count == k { return small }
            
            if count < k {
                start = large
            } else {
                end = small
            }
        }
        
        start
    }
}