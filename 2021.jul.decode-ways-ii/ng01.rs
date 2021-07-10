/*
"*"
"1*9"
"1*9*"
"999999999"
"19"
"*9"
"*6"
"*6*"
"1*"
"1*1"
"2*"
"2*2"
"***"
"1*1*2"
"2*2*1"
*/
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut result = 1;
        let mut past = ' ';
        s.chars().for_each(|char| {
            match char {
                '1' => {
                    if past == '1' || past == '2' {
                        result += 1;
                    } else if past == '*' {
                        result += 2;
                    }
                    past = '1';
                },
                '2' => {
                    if past == '1' || past == '2' {
                        result += 1;
                    } else if past == '*' {
                        result += 2;
                    }
                    past = '2';
                },
                '*' => {
                    if past == '1' {
                        result *= 18;
                    } else if past == '2' {
                        result *= 15;
                    } else if past == '*' {
                        result *= 9;
                        result += 9 + 6;
                    } else {
                        result *= 9;
                    }
                    past = '*';
                },
                '0' | '3' | '4' | '5' | '6' => {
                    if past == '1' || past == '2' {
                        result += 1;
                    } else if past == '*' {
                        result += 2;
                    }
                    past = ' ';
                },
                _ => {
                    if past == '1' || past == '*' {
                        result += 1;
                    }
                    past = ' ';
                },
            }
        });
        
        result
    }
}