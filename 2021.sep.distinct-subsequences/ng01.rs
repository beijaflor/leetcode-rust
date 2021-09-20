// https://leetcode.com/submissions/detail/557930340/
struct Solver {
    target: Vec<char>,
}

impl Solver {
    fn new(t: String) -> Self {
        Solver {
            target: t.chars().collect(),
        }
    }
    
    fn seek(&self, s: &Vec<char>, progress: usize, pointer: usize) -> i32 {
        let mut result = 0;
        let char = s[progress];
        // println!("{} {} {}", char, progress, pointer);
        if progress == s.len() - 1 {
            (pointer..self.target.len()).for_each(|index| {
                if self.target[index] == char {
                    result += 1;
                }
            });
        } else {
            (pointer..self.target.len()).for_each(|index| {
                if self.target[index] == char {
                    result += self.seek(s, progress + 1, index + 1);
                }
            });
        }
        result
    }
}


impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        // println!("{:?}", s);
        let mut solver = Solver::new(s);
        solver.seek(&t.chars().collect(), 0, 0)
    }
}