// https://leetcode.com/submissions/detail/460179016/
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut result = 0;
        let mut q: Vec<i32> = Vec::new();
        s.chars().for_each(|c| {
            if c == '(' {
                q.push(0);
            }
            if c == ')' {
                if let Some(v) = q.pop() {
                    let v = if v == 0 {
                        1
                    } else {
                        v * 2
                    };
                  
                    if q.is_empty() {
                        result += v;
                    } else {
                        let index = q.len() - 1;
                        q[index] += v;
                    }
                }
            }
        });
        result
    }
}