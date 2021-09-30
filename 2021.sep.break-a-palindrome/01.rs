// https://leetcode.com/submissions/detail/562629594/
impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        if palindrome.len() == 1 {
            String::from("")
        } else {
            let mut chars = palindrome.chars().collect::<Vec<char>>();
            
            for index in 0..(chars.len() / 2) {
                // println!("{}", index);
                if chars[index] != 'a' {
                    // println!("{}", chars[index]);
                    chars[index] = 'a';
                    // println!("{}", chars[index]);
                    // println!("{:?}", chars);
                    return chars.into_iter().collect()
                }
            }
            
            let len = chars.len() - 1;
            chars[len] = 'b';        
            chars.into_iter().collect()
        }
    }
}