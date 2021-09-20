// https://leetcode.com/submissions/detail/557935198/
use std::collections::HashMap;

struct Solver {
    string: Vec<char>,
    target: Vec<char>,
    memo: HashMap<(usize, usize), i32>,
}

impl Solver {
    fn new(s: String, t: String) -> Self {
        Solver {
            string: s.chars().collect(),
            target: t.chars().collect(),
            memo: HashMap::new(),
        }
    }
    
    fn seek(&mut self, progress: usize, pointer: usize) -> i32 {
        if let Some(memo) = self.memo.get(&(progress, pointer)) {
            return *memo
        }

        let mut result = 0;
        let char = self.target[progress];
        // println!("{} {} {}", char, progress, pointer);
        if progress == self.target.len() - 1 {
            (pointer..self.string.len()).for_each(|index| {
                if self.string[index] == char {
                    result += 1;
                }
            });
        } else {
            (pointer..self.string.len()).for_each(|index| {
                if self.string[index] == char {
                    result += self.seek(progress + 1, index + 1);
                }
            });
        }
        self.memo.insert((progress, pointer), result);       
        result
    }
}


impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        // println!("{:?}", s);
        let mut solver = Solver::new(s, t);
        solver.seek(0, 0)
    }
}