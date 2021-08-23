//
use std::collections::VecDeque;

fn determine(x: usize, y: usize, target: char, possibility: &mut Vec<Vec<Vec<char>>>, queue: &mut VecDeque<(usize, usize)>) {
    (0..9).for_each(|xx| {
        if let Some(pos) = possibility[y][xx].iter().position(|a| a == &target) {
            possibility[y][xx].remove(pos);
            if possibility[y][xx].len() == 1 {
                queue.push_back((xx, y));
            }
        }
    });

    (0..9).for_each(|yy| {
        if let Some(pos) = possibility[yy][x].iter().position(|a| a == &target) {
            possibility[yy][x].remove(pos);
            if possibility[yy][x].len() == 1 {
                queue.push_back((x, yy));
            }
        }
    });

    let offset_x = (x / 3) * 3;
    let offset_y = (y / 3) * 3;
    (0..9).for_each(|z| {
        let xx = offset_x + z % 3;
        let yy = offset_y + z / 3;
        if let Some(pos) = possibility[yy][xx].iter().position(|a| a == &target) {
            possibility[yy][xx].remove(pos);
            if possibility[yy][xx].len() == 1 {
                queue.push_back((xx, yy));
            }
        }
    });
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut possibility: Vec<Vec<Vec<char>>> = vec![vec![vec!['1','2','3','4','5','6','7','8','9']; 9]; 9];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        
        (0..9).for_each(|y| {
            (0..9).for_each(|x| {
                if board[y][x] != '.' {
                    possibility[y][x] = vec![];
                    determine(x, y, board[y][x], &mut possibility, &mut queue);
                }
            });
        });
        
        println!("queue: {:?}", queue);
        println!("possibility {:?}", possibility);
        
        while let Some((x, y)) = queue.pop_front() {
            println!("{:?} at {} / {}", possibility[y][x], x, y);
            let val = possibility[y][x].pop().unwrap();
            board[y][x] = val;
            determine(x, y, val, &mut possibility, &mut queue);
        }

        println!("possibility {:?}", possibility);
    }
}