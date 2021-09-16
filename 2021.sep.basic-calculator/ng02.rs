/*
"1 + 1"
" 2-1 + 2 "
"(1+(4+5+2)-3)+(6+8)"
"(1-(4+5+2)-3)+(6+8)"
"2147483647"
*/
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<(bool, i32)> = vec![];
        let mut current = (false, 0);
        let mut is_minas = false;
        s.chars().for_each(|char| {
            match char {
                '-' => {
                    current.0 = true;
                },
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if current.0 {
                        current.0 = false;
                        current.1 -= (char as u8 - '0' as u8) as i32
                    } else {
                        current.0 = false;
                        current.1 += (char as u8 - '0' as u8) as i32
                    }
                }
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
        current.1
    }
}