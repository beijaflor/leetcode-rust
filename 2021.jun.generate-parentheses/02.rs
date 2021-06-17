// https://leetcode.com/submissions/detail/508826584/
fn backtrack(result: &mut Vec<String>, current: String, open: i32, close: i32, max: i32) {
    if current.len() as i32 == max * 2 {
        result.push(current);
        return
    }
    
    if open < max {
        let current = format!("{}(", current);
        backtrack(result, current.clone(), open + 1, close, max);
        let current = current[0..(current.len() - 1)].to_string();
    }
    
    if close < open {
        let current = format!("{})", current);
        backtrack(result, current.clone(), open, close + 1, max);
        let current = current[0..(current.len() - 1)].to_string();
    }
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        backtrack(&mut result, "".to_string(), 0, 0, n);
        result
    }
}