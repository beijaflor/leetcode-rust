// https://leetcode.com/submissions/detail/580474870/
use std::collections::VecDeque;

fn dfs(board: &mut Vec<Vec<char>>, x: usize, y: usize) {
    let mut queue = VecDeque::<(usize, usize)>::new();
    queue.push_back((x, y));
    while let Some((x, y)) = queue.pop_front() {
        // println!("{} {}", x, y);
        if board[y][x] == 'O' {
            board[y][x] = '-';
            if y != 0 { queue.push_back((x, y - 1)) }
            if x != 0 { queue.push_back((x - 1, y)) }
            if y < board.len() - 1 { queue.push_back((x, y + 1)) }
            if x < board[0].len() - 1 { queue.push_back((x + 1, y)) }
        }
    }
}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        for x in 0..board[0].len() {
            if board[0][x] == 'O' {
                dfs(board, x, 0);
            }
            if board[board.len() - 1][x] == 'O' {
                dfs(board, x, board.len() - 1);
            }
        }

        for y in 1..board.len() - 1 {
            if board[y][0] == 'O' {
                dfs(board, 0, y);
            }
            if board[y][board[0].len() - 1] == 'O' {
                dfs(board, board[0].len() - 1, y);
            }
        }
        // println!("{:?}", board);
        
        for x in 0..board[0].len() {
            for y in 0..board.len() {
                if board[y][x] == '-' {
                    board[y][x] = 'O';
                } else {
                    board[y][x] = 'X';
                }
            }
        }
    }
}