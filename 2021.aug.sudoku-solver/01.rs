// https://leetcode.com/submissions/detail/542716043/
use std::collections::VecDeque;

struct Solver {
    solved: bool,
    // board: Vec<Vec<char>>,
    rows: Vec<Vec<i32>>,
    cols: Vec<Vec<i32>>,
    boxs: Vec<Vec<i32>>,
}

impl Solver {
    // fn new(&board: Vec<Vec<char>>) -> Self {
    fn new() -> Self {
        Solver {
            solved: false,
            // board: board,
            rows: vec![vec![0; 10]; 9],
            cols: vec![vec![0; 10]; 9],
            boxs: vec![vec![0; 10]; 9],
        }
    }
    
    fn check(&self, d: usize, row: usize, col: usize) -> bool {
        self.rows[row][d] + self.cols[col][d] + self.boxs[get_index(row, col)][d] == 0
    }
    
    fn place_number(&mut self, d: usize, row: usize, col: usize, board: &mut Vec<Vec<char>>) {
        self.rows[row][d] += 1;
        self.cols[col][d] += 1;
        self.boxs[get_index(row, col)][d] += 1;
        board[row][col] = ('0' as u8 + d as u8) as char;
    }
    
    fn remove_number(&mut self, d: usize, row: usize, col: usize, board: &mut Vec<Vec<char>>) {
        self.rows[row][d] -= 1;
        self.cols[col][d] -= 1;
        self.boxs[get_index(row, col)][d] -= 1;
        board[row][col] = '.';
    }
    
    fn place_next_number(&mut self, row: usize, col: usize, board: &mut Vec<Vec<char>>) {
        if col == 8 && row == 8 {
            self.solved = true;
        } else {
            if col == 8 {
                self.backtrack(row + 1, 0, board);
            } else {
                self.backtrack(row, col + 1, board);
            }
        }
    }
    
    fn backtrack(&mut self, row: usize, col: usize, board: &mut Vec<Vec<char>>) {
        if board[row][col] == '.' {
            for index in 1..10 {
                if self.check(index, row, col) {
                    self.place_number(index, row, col, board);
                    self.place_next_number(row, col, board);
                    if !self.solved {
                        self.remove_number(index, row, col, board);
                    }
                }
            }
        } else {
            self.place_next_number(row, col, board);
        }
    }
}

fn get_index(row: usize, col: usize) -> usize {
    (row / 3) * 3 + col / 3
}

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
        
        let mut solver = Solver::new();
        (0..9).for_each(|i| {
            (0..9).for_each(|j| {
                let num = board[i][j];
                if num != '.' {
                    let d = (num as u8 - '0' as u8) as usize;
                    // println!("{} {} {}", d, i, j);
                    solver.place_number(d, i, j, board);
                }
            });
        });
        solver.backtrack(0, 0, board);
    }
}