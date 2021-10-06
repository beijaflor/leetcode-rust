// https://leetcode.com/submissions/detail/565469615/
/**
 * // This is the BinaryMatrix's API interface.
 * // You should not implement it, or speculate about its implementation
 *  struct BinaryMatrix;
 *  impl BinaryMatrix {
 *     fn get(row: i32, col: i32) -> i32;
 *     fn dimensions() -> Vec<i32>;
 * };
 */

fn bsearch(binaryMatrix: &BinaryMatrix, row: i32, mut start: i32, mut end: i32) -> i32 {
    println!("row {}", row);
    let mut mid = (end + start) / 2;
    while start < end {
        println!("start {}, end {}, mid {}", start, end, mid);
        if binaryMatrix.get(row, mid) == 0 {
            start = mid + 1;
        } else {
            end = mid;
        }
        mid = (end + start) / 2;
    }
    mid
}

impl Solution {
    pub fn left_most_column_with_one(binaryMatrix: &BinaryMatrix) -> i32 {
        let matrix = binaryMatrix.dimensions();
        // println!("{:?}", matrix);
        let mut row = matrix[0] - 1;
        let mut col = matrix[1] - 1;

        if (0..=row).find(|row| {
            binaryMatrix.get(*row, 0) == 1
        }).is_some() { return 0 };
        
        let mut rows: Vec<i32> = (0..=row).filter(|row| {
            // println!("{}, {}, {}", row, col, binaryMatrix.get(*row, col));
            binaryMatrix.get(*row, col) == 1
        }).collect();
        if rows.len() == 0 {
            return -1
        }

        rows.into_iter().map(|row| {
            bsearch(binaryMatrix, row, 0, col)
        }).min().unwrap()
    }
}