// https://leetcode.com/submissions/detail/553382161/
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<(bool, i32)> = vec![];
        let mut digits: Vec<char> = vec![];
        let mut current = (false, 0);
        let mut is_minas = false;
        s.chars().for_each(|char| {
            // println!("{:?}", digits);
            // println!("{:?}, {:?}", current, stack);
            match char {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    digits.push(char);
                },
                _ => {
                    if !digits.is_empty() {
                        let num = digits.iter().collect::<String>().parse::<i32>().unwrap();
                        digits = vec![];
                        if current.0 {
                            current.1 -= num;
                        } else {
                            current.1 += num;
                        }
                        current.0 = false;
                    }
                }
            }

            match char {
                '-' => {
                    current.0 = true;
                },
                '(' => {
                    stack.push(current);
                    current = (false, 0);
                },
                ')' => {
                    let reveal = stack.pop().unwrap();
                    if reveal.0 {
                        current.1 = reveal.1 - current.1;
                    } else {
                        current.1 = reveal.1 + current.1;
                    }
                    current.0 = false;
                },
                _ => {}
            }
        });

        if !digits.is_empty() {
            let num = digits.iter().collect::<String>().parse::<i32>().unwrap();
            digits = vec![];
            if current.0 {
                current.1 -= num;
            } else {
                current.1 += num;
            }
        }

        current.1
    }
}