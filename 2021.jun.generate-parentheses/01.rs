// https://leetcode.com/submissions/detail/508823650/
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        if n == 0 {
            result.push("".to_string());
        } else {
            for index in 0..n {
                for left in Solution::generate_parenthesis(index) {
                    for right in Solution::generate_parenthesis(n - 1 - index) {
                        result.push(format!("({}){}", left, right));
                    }
                }
            }
        }
        result
    }
}