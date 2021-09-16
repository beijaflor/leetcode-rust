// https://leetcode.com/submissions/detail/555048804/
fn is_letter(c: &char) -> bool {
    ('a' <= *c && *c <= 'z') || ('A' <= *c && *c <= 'Z') 
}

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let mut lp = 0;
        let mut rp = chars.len() - 1;
        while lp < rp {
            println!("{} {} {} {}", chars[lp], chars[lp] as u8, chars[rp], chars[rp] as u8);
            if !is_letter(&chars[lp]) {
                lp += 1;
            } else if !is_letter(&chars[rp]) {
                rp -= 1;
            } else {
                chars.swap(lp, rp);
                lp += 1;
                rp -= 1;
            }
        }
        
        chars.into_iter().collect()
    }
}