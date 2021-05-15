// https://leetcode.com/submissions/detail/493373699/
#[derive(Debug)]
enum Mode {
    Sign,
    BDP,
    ADP,
    ExponentialSign,
    Exponential,
}

fn isDegit(char: &char) -> bool {
    &'0' <= char && char <= &'9'
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut mode: Mode = Mode::Sign;
        let mut valid = false;
        let invalid = s.chars().find(|char| {
            // println!("char: {:?}, mode: {:?}", char, mode);
            match mode {
                Mode::Sign => {
                    if char == &'-' || char == &'+' {
                        mode = Mode::BDP;
                        false
                    } else
                    if char == &'.' {
                        mode = Mode::ADP;
                        false
                    } else {
                        mode = Mode::BDP;
                        if isDegit(char) {
                            valid = true;
                            false
                        } else {
                            true
                        }
                    }
                },
                Mode::BDP => {
                    if char == &'e' || char == &'E' {
                        if !valid { return true }
                        mode = Mode::ExponentialSign;
                        valid = false;
                        false
                    } else 
                    if char == &'.' {
                        mode = Mode::ADP;
                        false
                    } else {
                        if isDegit(char) {
                            valid = true;
                            false
                        } else {
                            true
                        }
                    }
                },
                Mode::ADP => {
                    if char == &'e' || char == &'E' {
                        if !valid { return true }
                        mode = Mode::ExponentialSign;
                        valid = false;
                        false
                    } else {
                        if isDegit(char) {
                            valid = true;
                            false
                        } else {
                            true
                        }
                    }
                },
                Mode::ExponentialSign => {
                    if char == &'-' || char == &'+' {
                        false 
                    } else {
                        mode = Mode::Exponential;
                        if isDegit(char) {
                            valid = true;
                            false
                        } else {
                            true
                        }
                    }
                },
                Mode::Exponential => {
                    if isDegit(char) {
                        valid = true;
                        false
                    } else {
                        true
                    }
                },
            }
        }).is_some();
        // println!("is invalid: {}, mode: {:?}\n", invalid, mode);
        valid && !invalid
    }
}