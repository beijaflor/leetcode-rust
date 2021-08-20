// https://leetcode.com/submissions/detail/541407540/
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<char>> = (0..9).map(|_| HashSet::new()).collect();
        let mut cols: Vec<HashSet<char>> = (0..9).map(|_| HashSet::new()).collect();
        let mut boxs: Vec<HashSet<char>> = (0..9).map(|_| HashSet::new()).collect();

        for y in 0..9 {
            for x in 0..9 {
                let val = board[y][x];
                
                if val == '.' {
                    continue
                }
                
                if !rows[y].insert(val) {
                    return false
                }
                
                if !cols[x].insert(val) {
                    return false
                }
                
                let box_index = (y / 3) * 3 + x / 3;
                if !boxs[box_index].insert(val) {
                    return false
                }
                // println!("{} / {} = {}", x, y, val);
                // println!("box: {}", box_index);
            }
        }
        
        true
    }
}