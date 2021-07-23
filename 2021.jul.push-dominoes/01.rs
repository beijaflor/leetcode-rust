// https://leetcode.com/submissions/detail/526058982/
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut state: i32 = -1;
        let mut chars: Vec<char> = dominoes.chars().collect();
        for index in 0..dominoes.len() {
            match chars[index] {
                'L' => {
                    if state == -1 {
                        for index in (0..index).rev() {
                            if chars[index] == '.' {
                                chars[index] = 'L'
                            } else {
                                break
                            }
                        };
                    } else {
                        // println!("state: {}", state);
                        (0..(state / 2)).for_each(|offset| {
                            // println!("offset {}", offset);
                            chars[index - offset as usize - 1] = 'L';
                        });
                        if state % 2 == 1 {
                            // println!("odd!");
                            chars[index - (state / 2) as usize - 1] = '.';
                        }
                        state = -1;
                    }
                },
                'R' => {
                    state = 0;
                },
                _ => {
                    if state >= 0 {
                        chars[index] = 'R';
                        state += 1;
                    }
                },
            }
        }
        chars.into_iter().collect::<String>()
    }
}