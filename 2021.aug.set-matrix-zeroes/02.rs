// https://leetcode.com/submissions/detail/537974945/
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut flag_row = false;
        let mut flag_col = false;
        
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                if matrix[y][x] == 0 {
                    if y == 0 { flag_row = true }
                    if x == 0 { flag_col = true }
                    matrix[y][0] = 0;
                    matrix[0][x] = 0;
                }
            }
        }

        for y in (1..matrix.len()).rev() {
            for x in (1..matrix[y].len()).rev() {
                if matrix[y][0] == 0 || matrix[0][x] == 0 {
                    matrix[y][x] = 0;
                }
            }
        }
        
        // println!("flag_row: {}, flag_col: {}", flag_row, flag_col);
        
        if flag_row {
            for x in (0..matrix[0].len()) {
                matrix[0][x] = 0;
            }
        }

        if flag_col {
            for y in (0..matrix.len()) {
                matrix[y][0] = 0;
            }
        }
    }
}