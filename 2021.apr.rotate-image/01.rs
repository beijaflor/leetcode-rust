// https://leetcode.com/submissions/detail/485065128
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        for offset in 0..((len + 1) / 2) {
            // println!("offset: {}", offset);
            for index in 0..(len / 2) {
                // println!("index: {}", index);
                let cache = matrix[len - 1 - index][offset];
                matrix[len - 1 - index][offset] = matrix[len - 1 - offset][len - 1 - index];
                matrix[len - 1 - offset][len - 1 - index] = matrix[index][len - 1 - offset];
                matrix[index][len - 1 - offset] = matrix[offset][index];
                matrix[offset][index] = cache;
            }
        }
        // println!("{:?}", matrix);
    }
}