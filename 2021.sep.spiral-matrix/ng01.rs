/*
[[1]]
[[1,2]]
[[1],[2]]
*/
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut top = 0;
        let mut bottom = matrix.len() - 1;
        let mut left = 0;
        let mut right = matrix[0].len() - 1;
        let mut x = 0;
        let mut y = 0;
        let mut result: Vec<i32> = vec![];
        loop {
            while x < right {
                println!("{} {}", x, y);
                result.push(matrix[y][x]);
                x += 1;
            }
            if !(top < bottom) {
                break
            }
            top += 1;

            println!("{} {}", x, y);
            result.push(matrix[y][x]);
            y += 1;

            while y < bottom {
                println!("{} {}", x, y);
                result.push(matrix[y][x]);
                y += 1;
            }
            if !(left < right) {
                break
            }
            right -= 1;

            println!("{} {}", x, y);
            result.push(matrix[y][x]);
            x -= 1;
            
            while x > left {
                println!("{} {}", x, y);
                result.push(matrix[y][x]);
                x -= 1;
            }
            if !(top < bottom) {
                break
            }
            bottom -= 1;

            println!("{} {}", x, y);
            result.push(matrix[y][x]);
            y -= 1;

            while y > top {
                println!("{} {}", x, y);
                result.push(matrix[y][x]);
                y -= 1;
            }
            if !(left < right) {
                break
            }
            right += 1;

            println!("{} {}", x, y);
            result.push(matrix[y][x]);
            x += 1;
        }
        result
    }
}