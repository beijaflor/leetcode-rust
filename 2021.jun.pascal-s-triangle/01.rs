// https://leetcode.com/submissions/detail/511098161/
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle: Vec<Vec<i32>> = vec![vec![1; 1]; 1];
        (1..num_rows).for_each(|index| {
            // println!("triangle: {:?}", triangle);
            let index = index as usize;
            let mut row: Vec<i32> = Vec::with_capacity(index);
            (0..=index).for_each(|j| {
                if j == 0 {
                    row.push(triangle[index - 1][0]);
                } else if j == index {
                    row.push(triangle[index - 1][j - 1]);
                } else {
                    row.push(triangle[index - 1][j - 1] + triangle[index - 1][j]);
                }
            });
            // println!("row: {:?}", row);
            triangle.push(row);
        });
        triangle
    }
}