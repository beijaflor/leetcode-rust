// https://leetcode.com/submissions/detail/571443489/
fn find_row(image: &Vec<Vec<char>>, row: usize) -> Option<(usize, usize)> {
    if let Some(start) = image[row].iter().position(|col| *col == '1') {
        let end = image[row].len() - image[row].iter().rev().position(|col| *col == '1').unwrap() - 1;
        Some((start, end))
    } else {
        None
    }
}

fn find_col(image: &Vec<Vec<char>>, col: usize) -> Option<(usize, usize)> {
    if let Some(start) = image.iter().position(|row| row[col] == '1') {
        let end = image.len() - image.iter().rev().position(|row| row[col] == '1').unwrap() - 1;
        Some((start, end))
    } else {
        None
    }
}

impl Solution {
    pub fn min_area(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
        let (y, x) = (x as usize, y as usize);
        let mut rows = (x, x);
        let mut cols = (y, y);
        
        // println!("rows {:?}", rows);
        // println!("cols {:?}", cols);

        for row in (0..image.len()) {
            if let Some(new_rows) = find_row(&image, row) {
                rows = (rows.0.min(new_rows.0), rows.1.max(new_rows.1));
            }
        }

        for col in (0..image[y].len()) {
            if let Some(new_cols) = find_col(&image, col) {
                cols = (cols.0.min(new_cols.0), cols.1.max(new_cols.1));
            }
        }

        // println!("rows {:?}", rows);
        // println!("cols {:?}", cols);

        ((rows.1 - rows.0 + 1) * (cols.1 - cols.0 + 1)) as i32
    }
}