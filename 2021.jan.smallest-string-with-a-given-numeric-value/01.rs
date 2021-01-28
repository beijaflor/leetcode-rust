// https://leetcode.com/submissions/detail/448976496/
const A: u8 = 'a' as u8;

impl Solution {
    pub fn get_smallest_string(mut n: i32, mut k: i32) -> String {
        let mut result: Vec<char> = Vec::new();
        
        if k - n > 25 {
            loop {
                k -= 26;
                n -= 1;
                result.push('z');
                if k - n < 25 { break }
            }
        }

        if n == 0 {
            return result.iter().collect()
        }

        let mut result2: Vec<char> = (0..n - 1).map(|_| 'a').collect();
        result2.push((A + (k - n) as u8) as char);
        result2.append(&mut result);

        result2.iter().collect()
    }
}