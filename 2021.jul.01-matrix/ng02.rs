fn around(mat: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<i32> {
    // println!("x{}, y{}", x, y);
    let mut result: Vec<i32> = Vec::new();
    if x != 0 {
        // if y != 0 {
        //     result.push(mat[y - 1][x - 1]);
        // }
        result.push(mat[y][x - 1]);
        // if y != mat.len() - 1 {
        //     result.push(mat[y + 1][x - 1]);
        // }
    }
    if y != 0 {
        result.push(mat[y - 1][x]);
    }
    if y != mat.len() - 1 {
        result.push(mat[y + 1][x]);
    }
    if x != mat[0].len() - 1 {
        // if y != 0 {
        //     result.push(mat[y - 1][x + 1]);
        // }
        result.push(mat[y][x + 1]);
        // if y != mat.len() - 1 {
        //     result.push(mat[y + 1][x + 1]);
        // }
    }
    result
}


impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![vec![i32::MAX; mat[0].len()]; mat.len()];
        
        loop {
            let mut flag = true;
            (0..mat.len()).for_each(|y| {
                (0..mat[0].len()).for_each(|x| {
                    if mat[y][x] == 0 {
                        result[y][x] = 0;
                    } else {
                        result[y][x] = *around(&result, x, y).iter().min().unwrap();
                        if result[y][x] == i32::MAX {
                            flag = false;
                        } else {
                            result[y][x] += 1;
                        }
                    } 
                });
            });
            
            if flag { break }
        }

        (0..mat.len()).for_each(|y| {
            (0..mat[0].len()).for_each(|x| {
                if mat[y][x] == 0 {
                    result[y][x] = 0;
                } else {
                    result[y][x] = around(&result, x, y).iter().min().unwrap() + 1;
                } 
            });
        });

        result
    }
}