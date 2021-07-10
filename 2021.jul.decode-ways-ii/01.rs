// https://leetcode.com/submissions/detail/520162496/
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let M = 1_000_000_007;
        let chars = s.chars().collect::<Vec<char>>();
        let mut first: i64 = 1;
        let mut second: i64 = match chars[0] {
            '*' => 9,
            '0' => 0,
            _ => 1,
        };

        for index in (1..chars.len()) {
            let mut temp: i64 = second;
            if chars[index] == '*' {
                second = 9 * second % M;
                match chars[index - 1] {
                    '1' => second = (second + 9 * first) % M,
                    '2' => second = (second + 6 * first) % M,
                    '*' => second = (second + 15 * first) % M,
                    _ => {},
                }
            } else {
                second = if chars[index] != '0' { second } else { 0 };
                match chars[index - 1] {
                    '1' => second = (second + first) % M,
                    '2' => {
                        if chars[index] <= '6' {
                            second = (second + first) % M;
                        }
                    },
                    '*' => {
                        let add = if chars[index] <= '6' { 2 } else { 1 };
                        second = (second + add * first) % M;
                    },
                    _ => {},
                }
            }
            first = temp;
        }

        second as i32
    }
}