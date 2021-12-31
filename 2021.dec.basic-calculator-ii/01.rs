// https://leetcode.com/submissions/detail/606826095/
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut total = 0;
        let mut last = 0;
        let mut current = 0;
        let mut operator = '+';
        for char in s.chars() {
            // println!("{} {} {} {}", char, current, last, total);
            match char {
                '+' | '-' | '/' | '*' => {
                    match operator {
                        '+' => {
                            total += last;
                            last = current;
                        },
                        '-' => {
                            total += last;
                            last = -current;
                        },
                        '/' => { last /= current },
                        '*' => { last *= current },
                        _ => {},
                    }
                    operator = char;
                    current = 0;
                },
                '0' ..= '9' => {
                    current = current * 10 + (char as u8 - '0' as u8) as i32;
                    // println!("{}", current);
                },
                _ => {}
            }
            // println!("{}", char);
        }
        
        // println!("{} {}", total, last);

        total + match operator {
            '+' => { last + current },
            '-' => { last - current },
            '/' => { last / current },
            '*' => { last * current },
            _ => { 0 },
        }
    }
}