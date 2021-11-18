// https://leetcode.com/submissions/detail/571438761/
fn find_row(image: &Vec<Vec<char>>, row: usize) -> (usize, usize) {
    if let Some(start) = image[row].iter().position(|col| *col == '1') {
        let end = image[row].len() - image[row].iter().rev().position(|col| *col == '1').unwrap() - 1;
        (start, end)
    } else {
        panic!("panic!!")
    }
}

fn find_col(image: &Vec<Vec<char>>, col: usize) -> (usize, usize) {
    if let Some(start) = image.iter().position(|row| row[col] == '1') {
        let end = image.len() - image.iter().rev().position(|row| row[col] == '1').unwrap() - 1;
        (start, end)
    } else {
        panic!("panic!!")
    }
}

impl Solution {
    pub fn min_area(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
        let (y, x) = (x as usize, y as usize);
        let mut rows = find_row(&image, y);
        let mut cols = find_col(&image, x);
        
        // println!("rows {:?}", rows);
        // println!("cols {:?}", cols);

        for row in (0..y).rev() {
            if image[row][rows.0] == '1' || image[row][rows.1] == '1' {
                let new_rows = find_row(&image, row);
                rows = (rows.0.min(new_rows.0), rows.1.max(new_rows.1));
            }
        }

        for row in ((y + 1)..image.len()) {
            if image[row][rows.0] == '1' || image[row][rows.1] == '1' {
                let new_rows = find_row(&image, row);
                rows = (rows.0.min(new_rows.0), rows.1.max(new_rows.1));
            }
        }

        for col in (0..x).rev() {
            if image[cols.0][col] == '1' || image[cols.1][col] == '1' {
                let new_cols = find_col(&image, col);
                cols = (cols.0.min(new_cols.0), cols.1.max(new_cols.1));
            }
        }

        for col in ((x + 1)..image[y].len()) {
            if image[cols.0][col] == '1' || image[cols.1][col] == '1' {
                let new_cols = find_col(&image, col);
                cols = (cols.0.min(new_cols.0), cols.1.max(new_cols.1));
            }
        }

        // println!("rows {:?}", rows);
        // println!("cols {:?}", cols);

        ((rows.1 - rows.0 + 1) * (cols.1 - cols.0 + 1)) as i32
    }
}