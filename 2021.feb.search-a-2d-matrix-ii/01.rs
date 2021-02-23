// https://leetcode.com/submissions/detail/459699492/
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut pointer = 0;
        let mut max = usize::max(matrix.len(), matrix[0].len());
        while pointer < max {
            // println!("{}", matrix[pointer][pointer]);
            let pointer_y = usize::min(pointer, matrix.len() - 1);
            let pointer_x = usize::min(pointer, matrix[0].len() - 1);
            if matrix[pointer_y][pointer_x] == target {
                return true
            }
            if matrix[pointer_y][pointer_x] > target {
                for index in 0..pointer {
                    if matrix[pointer_y][usize::min(pointer_x, index)] == target  || matrix[usize::min(pointer_y, index)][pointer_x] == target {
                        return true
                    }
                }
            }
            pointer += 1;
        }
        false
    }
}