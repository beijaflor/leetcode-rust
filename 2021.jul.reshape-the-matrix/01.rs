// https://leetcode.com/submissions/detail/517729360/
impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let r = r as usize;
        let c = c as usize;
        if mat.len() * mat[0].len() != (r * c) as usize {
            return mat
        }
        let mut result: Vec<Vec<i32>> = vec![vec![]; r];
        let mut x = 0;
        let mut y = 0;
        mat.into_iter().for_each(|row| {
            row.into_iter().for_each(|col| {
                result[y].push(col);
                if !(result[y].len() < c) {
                    y += 1;
                    x = 0;
                } else {
                    x += 1;
                }
            });
        });
        result
    }
}