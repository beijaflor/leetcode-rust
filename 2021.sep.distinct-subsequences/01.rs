// https://leetcode.com/submissions/detail/557939992/
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
        if progress == self.string.len()
           || pointer == self.target.len()
           || self.string.len() - progress < self.target.len() - pointer
        {
            return if pointer == self.target.len() { 1 } else { 0 }
        }
        
        if let Some(memo) = self.memo.get(&(progress, pointer)) {
            return *memo
        }

        let mut result = self.seek(progress + 1, pointer);
        // println!("{} {} {}", char, progress, pointer);
        if self.string[progress] == self.target[pointer] {
            result += self.seek(progress + 1, pointer + 1);
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