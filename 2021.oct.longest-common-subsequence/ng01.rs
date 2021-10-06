/*
"oxcpqrsvwf"
"shmtulqrypy"
"psnw"
"vozsh"
"abcde"
"ace"
"abcdef"
"defabc"
"defabc"
"abcdef"
*/
use std::collections::HashMap;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut result = 0;
        let mut p1 = 0;
        let mut p2 = 0;
        let (text1, text2) = if text1.len() < text2.len() { (text1, text2) } else { (text2, text1) };
        let mut chars1 = text1.chars().collect::<Vec<char>>();
        let mut chars2 = text2.chars().collect::<Vec<char>>();
        while p1 < chars1.len() && p2 < chars2.len() {
            let mut p2_tmp = p2;
            while p2_tmp < chars2.len() {
                if chars2[p2_tmp] == chars1[p1] {
                    result += 1;
                    p2 = p2_tmp + 1;
                    break
                }
                p2_tmp += 1;
            }
            p1 += 1;
        }
        result
    }
}