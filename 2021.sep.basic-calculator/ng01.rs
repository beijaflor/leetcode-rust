/*
"1 + 1"
" 2-1 + 2 "
"(1+(4+5+2)-3)+(6+8)"
"(1-(4+5+2)-3)+(6+8)"
*/
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut is_minas = false;
        s.chars().fold(0, |acc, char| {
            match char {
                '-' => {
                    is_minas = true;
                    acc
                },
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if is_minas {
                        is_minas = false;
                        acc - (char as u8 - '0' as u8) as i32
                    } else {
                        is_minas = false;
                        acc + (char as u8 - '0' as u8) as i32
                    }
                }
                _ => {
                    acc
                }
            }
        })
    }
}