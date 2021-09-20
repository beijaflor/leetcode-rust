// https://leetcode.com/submissions/detail/557069522/
struct Solver {
    result: Vec<String>,
    digits: Vec<char>,
    target: i64,
    len: usize,
}

impl Solver {
    fn new(target: i64, digits: Vec<char>) -> Self {
        let len = digits.len();
        Solver {
            result: vec![],
            digits: digits,
            target: target,
            len: len,
        }
    }
    
    fn rec(&mut self, index: usize, prev: i64, current: i64, val: i64, ops: &mut Vec<String>) {
        let mut nums = self.digits.clone();
        
        if index == nums.len() {
            // println!("{:?} {} {}", ops, val, current);
            if val == self.target && current == 0 {
                self.result.push(ops[1..ops.len()].join(""));
            }
            return
        }
        
        let parsed_num = (nums[index] as u8 - '0' as u8) as i64;
        let current = current * 10i64 +  parsed_num;
        // let current_rep = current.to_string();
        
        if current > 0 {
            self.rec(index + 1, prev, current, val, ops);
        }
        let mut new_ops = ops.clone();
        new_ops.push(String::from("+"));
        new_ops.push(current.to_string());
        self.rec(index + 1, current, 0, val + current, &mut new_ops);
        
        if ops.len() > 0 {
            let mut new_ops = ops.clone();
            new_ops.push(String::from("-"));
            new_ops.push(current.to_string());
            self.rec(index + 1, -current, 0, val - current, &mut new_ops);
        
            let mut new_ops = ops.clone();
            new_ops.push(String::from("*"));
            new_ops.push(current.to_string());
            self.rec(index + 1, prev * current, 0, val - prev + (prev * current), &mut new_ops);
        }
        
    }
}


impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        if num.len() == 0 { return vec![] }
        let mut solver = Solver::new(target as i64, num.chars().collect());
        solver.rec(0, 0i64, 0i64, 0i64, &mut vec![]);
        
        solver.result
    }
}