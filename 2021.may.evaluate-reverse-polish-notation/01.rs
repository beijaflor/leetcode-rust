// https://leetcode.com/submissions/detail/498061168/
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut calc: Vec<i32> = vec![];
        tokens.into_iter().for_each(|token| {
            match &*token {
                "+" => {
                    let b = calc.pop().unwrap();
                    let a = calc.pop().unwrap();
                    calc.push(a + b);
                },
                "-" => {
                    let b = calc.pop().unwrap();
                    let a = calc.pop().unwrap();
                    calc.push(a - b);
                },
                "*" => {
                    let b = calc.pop().unwrap();
                    let a = calc.pop().unwrap();
                    calc.push(a * b);
                },
                "/" => {
                    let b = calc.pop().unwrap();
                    let a = calc.pop().unwrap();
                    calc.push(a / b);
                },
                _ => {
                    calc.push(token.parse::<i32>().unwrap());
                }
            }
        });
        calc.pop().unwrap()
    }
}