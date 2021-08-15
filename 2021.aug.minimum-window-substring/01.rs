// https://leetcode.com/submissions/detail/538896803/
use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() == 0 || t.len() == 0 { return String::from("") }
        
        let mut dict: HashMap<char, i32> = HashMap::new();
        t.chars().for_each(|char| {
            dict.entry(char).and_modify(|c| *c += 1).or_insert(1);
        });
        
        let required = dict.len();
        
        let mut l = 0;
        let mut r = 0;
        
        let mut formed = 0;
        
        let mut window: HashMap<char, i32> = HashMap::new();
        let mut ans: (i32, usize, usize) = (-1, 0, 0);
        let chars: Vec<char> = s.chars().collect();
        
        while r < s.len() {
            let mut c = chars[r];
            window.entry(c).and_modify(|c| *c += 1).or_insert(1);
            
            if dict.get(&c).is_some() && window.get(&c) == dict.get(&c) {
                formed += 1;
            }
            
            while l <= r && formed == required {
                c = chars[l];
                
                if ans.0 == -1 || ((r - l + 1) as i32) < ans.0 {
                    ans.0 = (r - l + 1) as i32;
                    ans.1 = l;
                    ans.2 = r;
                }
                
                window.entry(c).and_modify(|c| *c -= 1);
                
                if dict.get(&c).is_some() && window.get(&c) < dict.get(&c) {
                    formed -= 1;
                }
                
                l += 1;
            }
            
            r += 1;
        }
        
        if ans.0 == -1 {
            String::from("")
        } else {
            s[ans.1..=ans.2].to_string()
        }
    }
}