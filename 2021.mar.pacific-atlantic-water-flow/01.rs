// https://leetcode.com/submissions/detail/472357387/
use std::collections::VecDeque;

impl Solution {
    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // prepare
        let y_len = matrix.len();
        if y_len == 0 { return vec![] }
        let x_len = matrix[0].len();
        if x_len == 0 { return vec![] }
        let mut pacific: Vec<Vec<bool>> = Vec::with_capacity(y_len);
        (0..y_len).for_each(|_| {
            let mut row: Vec<bool> = Vec::with_capacity(x_len);
            (0..x_len).for_each(|_| {
                row.push(false);
            });
            pacific.push(row);
        });
        let mut pacific_visited = pacific.clone();
        let mut atlantic = pacific.clone();
        let mut atlantic_visited = pacific.clone();

        // calc pacific
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        (0..x_len).for_each(|index| queue.push_back((index, 0)));
        (0..y_len).for_each(|index| queue.push_back((0, index)));
        
        while let Some((x, y)) = queue.pop_front() {
            // println!("x: {}, y: {}", x, y);
            pacific[y][x] = true;
            if !pacific_visited[y][x] {
                pacific_visited[y][x] = true;
                let val = matrix[y][x];
                // println!("{}", val);
                
                if x > 0 && !pacific_visited[y][x - 1] {
                    if matrix[y][x - 1] >= val {
                        queue.push_back((x - 1, y));
                    } else {
                        // pacific_visited[y][x - 1] = true;
                    }
                }

                if y > 0 && !pacific_visited[y - 1][x] {
                    if matrix[y - 1][x] >= val {
                        queue.push_back((x, y - 1));
                    } else {
                        // pacific_visited[y - 1][x] = true;
                    }
                }

                if x < x_len - 1 && !pacific_visited[y][x + 1] {
                    if matrix[y][x + 1] >= val {
                        queue.push_back((x + 1, y));
                    } else {
                        // pacific_visited[y][x + 1] = true;
                    }
                }

                if y < y_len - 1 && !pacific_visited[y + 1][x] {
                    if matrix[y + 1][x] >= val {
                        queue.push_back((x, y + 1));
                    } else {
                        // pacific_visited[y + 1][x] = true;
                    }
                }
            }
        }
        
        // println!("pacific: {:?}", pacific);
        // println!("{:?}", pacific_visited);

        // calc atlantic
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        (0..x_len).for_each(|index| queue.push_back((index, y_len - 1)));
        (0..y_len).for_each(|index| queue.push_back((x_len - 1, index)));
        
        // println!("{:?}", queue);
        
        while let Some((x, y)) = queue.pop_front() {
            // println!("x: {}, y: {}", x, y);
            atlantic[y][x] = true;
            if !atlantic_visited[y][x] {
                atlantic_visited[y][x] = true;
                let val = matrix[y][x];
                // println!("{}", val);
                
                if x > 0 && !atlantic_visited[y][x - 1] {
                    if matrix[y][x - 1] >= val {
                        queue.push_back((x - 1, y));
                    } else {
                        // atlantic_visited[y][x - 1] = true;
                    }
                }

                if y > 0 && !atlantic_visited[y - 1][x] {
                    if matrix[y - 1][x] >= val {
                        queue.push_back((x, y - 1));
                    } else {
                        // atlantic_visited[y - 1][x] = true;
                    }
                }

                if x < x_len - 1 && !atlantic_visited[y][x + 1] {
                    if matrix[y][x + 1] >= val {
                        queue.push_back((x + 1, y));
                    } else {
                        // atlantic_visited[y][x + 1] = true;
                    }
                }

                if y < y_len - 1 && !atlantic_visited[y + 1][x] {
                    if matrix[y + 1][x] >= val {
                        queue.push_back((x, y + 1));
                    } else {
                        // atlantic_visited[y + 1][x] = true;
                    }
                }
            }
        }
        
        // println!("atlantic: {:?}", atlantic);
        // println!("{:?}", atlantic_visited);

        let mut result: Vec<Vec<i32>> = Vec::new();

        (0..y_len).for_each(|y| {
            (0..x_len).for_each(|x| {
                if atlantic[y][x] && pacific[y][x] {
                    result.push(vec![y as i32, x as i32]);
                }
            });
        });

        // println!("{:?}", result);
        result
    }
}