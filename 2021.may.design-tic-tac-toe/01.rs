// https://leetcode.com/submissions/detail/496760452/
struct TicTacToe {
    board: Vec<Vec<i32>>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TicTacToe {

    /** Initialize your data structure here. */
    fn new(n: i32) -> Self {
        { TicTacToe { board: vec![vec![-1; n as usize]; n as usize] } }
    }
    
    /** Player {player} makes a move at ({row}, {col}).
        @param row The row of the board.
        @param col The column of the board.
        @param player The player, can be either 1 or 2.
        @return The current winning condition, can be either:
                0: No one wins.
                1: Player 1 wins.
                2: Player 2 wins. */
    fn make_a_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        let row = row as usize;
        let col = col as usize;
        self.board[row][col] = player;
        if !self.board[row].iter().find(|x| *x != &player).is_some() {
            player
        } else
        if !(0..self.board.len()).find(|i| self.board[*i][col] != player).is_some() {
            player
        } else
        if row == col && !(0..self.board.len()).find(|i| self.board[*i][*i] != player).is_some() {
            player
        } else
        if row == self.board.len() - col - 1 && !(0..self.board.len()).find(|i| self.board[*i][self.board.len() - *i - 1] != player).is_some() {
            player
        }
        else {
            0
        }
    }
}

/**
 * Your TicTacToe object will be instantiated and called as such:
 * let obj = TicTacToe::new(n);
 * let ret_1: i32 = obj.make_a_move(row, col, player);
 */