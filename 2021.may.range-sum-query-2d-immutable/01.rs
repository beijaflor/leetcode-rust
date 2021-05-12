// https://leetcode.com/submissions/detail/492127044/
struct NumMatrix {
    matrix: Vec<Vec<i32>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        NumMatrix { matrix: matrix }
    }
    
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let col1 = col1 as usize;
        let row2 = row2 as usize + 1;
        let col2 = col2 as usize + 1;
        // println!("{}, {}, {}, {}", row1, col1, row2, col2);
        self.matrix[row1..row2].iter().fold(0, |acc, row| {
            let sum: i32 = row[col1..col2].iter().sum();
            acc + sum
        })
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */