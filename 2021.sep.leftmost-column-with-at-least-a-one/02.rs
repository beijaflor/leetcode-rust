// https://leetcode.com/submissions/detail/565477478/
/**
 * // This is the BinaryMatrix's API interface.
 * // You should not implement it, or speculate about its implementation
 *  struct BinaryMatrix;
 *  impl BinaryMatrix {
 *     fn get(row: i32, col: i32) -> i32;
 *     fn dimensions() -> Vec<i32>;
 * };
 */

impl Solution {
    pub fn left_most_column_with_one(binaryMatrix: &BinaryMatrix) -> i32 {
        let matrix = binaryMatrix.dimensions();
        // println!("{:?}", matrix);
        let mut row = matrix[0] - 1;
        let mut col = matrix[1] - 1;

        if !(0..=row).find(|row| { binaryMatrix.get(*row, col) == 1 }).is_some() {
            return -1
        }
        col -= 1;

        while col >= 0 && row >= 0 {
            // println!("{}, {}", col, binaryMatrix.get(row, col));
            if binaryMatrix.get(row, col) == 0 {
                row -= 1;
            } else {
                col -= 1;
            }
        }
        col + 1
    }
}