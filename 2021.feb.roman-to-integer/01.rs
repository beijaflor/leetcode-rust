// https://leetcode.com/submissions/detail/458439494/
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut chars: Vec<char> = s.chars().collect();
        let s1 = ['I', 'X', 'C', 'M'];
        let s2 = ['V', 'L', 'D'];
        let v = [1, 10, 100, 1000];
        let mut pointer = chars.len() - 1;
        loop {
            let char = chars[pointer];
            if let Some(pos) = s1.iter().position(|v| *v == char) {
                result += v[pos];
                if pointer == 0 {
                    break
                } else { pointer -= 1 };

                if pos > 0 && chars[pointer] == s1[pos - 1] {
                    result -= v[pos - 1];
                    if pointer == 0 { break } else { pointer -= 1 };
                }
            } else if let Some(pos) = s2.iter().position(|v| *v == char) {
                result += v[pos] * 5;
                if pointer == 0 { break } else { pointer -= 1 };
                if chars[pointer] == s1[pos] {
                    result -= v[pos];
                    if pointer == 0 { break } else { pointer -= 1 };
                }
            }
        }
        result
    }
}