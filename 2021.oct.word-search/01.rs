// https://leetcode.com/submissions/detail/567064329/
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

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let chars = word.chars().collect::<Vec<char>>();
        let first_letter = chars[0];
        let mut initials = Vec::<(usize, usize)>::new();
        (0..board.len()).for_each(|row| {
            (0..board[0].len()).for_each(|col| {
                if board[row][col] == first_letter {
                    initials.push((row, col));
                }
            });
        });
        // println!("initals: {:?}", initials);
        for pos in initials.into_iter() {
            let mut visits: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
            if dfs(pos, &mut visits, &board, &chars, 0) {
                return true
            }
        }

        false
    }
}