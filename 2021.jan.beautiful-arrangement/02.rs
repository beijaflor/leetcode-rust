// https://leetcode.com/submissions/detail/530469754/
fn around(mat: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    if x != 0 {
        result.push(mat[y][x - 1]);
    }
    if y != 0 {
        result.push(mat[y - 1][x]);
    }
    if y != mat.len() - 1 {
        result.push(mat[y + 1][x]);
    }
    if x != mat[0].len() - 1 {
        result.push(mat[y][x + 1]);
    }
    result
}


impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![vec![i32::MAX; mat[0].len()]; mat.len()];
        
        (0..mat.len()).for_each(|y| {
            (0..mat[0].len()).for_each(|x| {
                let mut new_val = 0;
                if mat[y][x] == 1 {
                    new_val = *around(&result, x, y).iter().min().unwrap();
                    if new_val != i32::MAX {
                        new_val += 1;
                    }
                }
                result[y][x] = new_val;
            });
        });

        (0..mat.len()).rev().for_each(|y| {
            (0..mat[0].len()).rev().for_each(|x| {
                let mut new_val = 0;
                if mat[y][x] == 1 {
                    new_val = *around(&result, x, y).iter().min().unwrap();
                    if new_val != i32::MAX {
                        new_val += 1;
                    }
                }
                result[y][x] = new_val;
            });
        });

        result
    }
}