/*
[[1,1,1,1,1],[0,0,0,1,1],[0,0,1,1,1],[0,0,0,0,1],[0,0,0,0,0]]
[[0,0],[1,1]]
[[0,0],[0,0]]
[[0,0,0,1],[0,0,1,1],[0,1,1,1]]
*/
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
        println!("{:?}", matrix);
        let mut row = matrix[0] - 1;
        let mut col = matrix[1] - 1;
        if binaryMatrix.get(row, col) == 0 {
            return -1
        }
        while col >= 0 {
            println!("{}, {}", col, binaryMatrix.get(row, col));
            if binaryMatrix.get(row, col) == 0 {
                return (col + 1) as i32
            }
            col -= 1;
        }
        
        0
    }
}