// https://leetcode.com/submissions/detail/555879892/
impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut x = 0;
        let mut y = 0;
        let mut direction = 0;
        let mut flag = 0;
        let mut result: Vec<i32> = vec![matrix[0][0]];
        matrix[0][0] = i32::MIN;
        loop {
            // println!("{} {} / direction {}, flag {}", x, y, direction, flag);
            // println!("{:?}", matrix);
            match direction {
                0 => {
                    if x == matrix[0].len() - 1 || matrix[y][x + 1] == i32::MIN {
                        flag += 1;
                        direction = 1;
                    } else {
                        flag = 0;
                        x += 1;
                        result.push(matrix[y][x]);
                        matrix[y][x] = i32::MIN;
                    }
                },
                1 => {
                    if y == matrix.len() - 1 || matrix[y + 1][x] == i32::MIN {
                        flag += 1;
                        direction = 2;
                    } else {
                        flag = 0;
                        y += 1;
                        result.push(matrix[y][x]);
                        matrix[y][x] = i32::MIN;
                    }
                },
                2 => {
                    if x == 0 || matrix[y][x - 1] == i32::MIN {
                        flag += 1;
                        direction = 3;
                    } else {
                        flag = 0;
                        x -= 1;
                        result.push(matrix[y][x]);
                        matrix[y][x] = i32::MIN;
                    }
                },
                3 => {
                    if y == 0 || matrix[y - 1][x] == i32::MIN {
                        flag += 1;
                        direction = 0;
                    } else {
                        flag = 0;
                        y -= 1;
                        result.push(matrix[y][x]);
                        matrix[y][x] = i32::MIN;
                    }
                },
                _ => panic!("shouldn't reach this")
            }
            if flag > 3 { break }
        }

        // println!("");
        result
    }
}