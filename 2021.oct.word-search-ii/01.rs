// https://leetcode.com/submissions/detail/568113227/
fn dfs(mut pos: (usize, usize), visits: &mut Vec<Vec<bool>>, board: &Vec<Vec<char>>, chars: &Vec<char>, index: usize) -> bool {
    let (row, col) = pos;
    if visits[row][col] || board[row][col] != chars[index] {
        // println!("FALSE");
        return false
    }
    if index == chars.len() - 1 {
        // println!("TRUE");
        return true
    }
    // println!("{} / {}", row, col);
    visits[row][col] = true;
    if row > 0 {
        if dfs((row - 1, col), visits, board, chars, index + 1) {
            return true
        }
    }
    if col > 0 {
        if dfs((row, col - 1), visits, board, chars, index + 1) {
            return true
        }
    }
    if row < board.len() - 1 {
        if dfs((row + 1, col), visits, board, chars, index + 1) {
            return true
        }
    }
    if col < board[0].len() - 1 {
        if dfs((row, col + 1), visits, board, chars, index + 1) {
            return true
        }
    }
    visits[row][col] = false;
    false
}

fn exist(board: &Vec<Vec<char>>, word: &String) -> bool {
    let chars = word.chars().collect::<Vec<char>>();
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            let mut visits: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
            if dfs((row, col), &mut visits, &board, &chars, 0) {
                return true
            }
        }
    }
    false
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        words.into_iter().filter(|word| {
            exist(&board, word)
        }).collect()
    }
}