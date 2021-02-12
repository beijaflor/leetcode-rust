// https://leetcode.com/submissions/detail/455035422/
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s = s.chars().collect::<Vec<char>>();
        s.sort();
        let mut t = t.chars().collect::<Vec<char>>();
        t.sort();
        // println!("{:?}", s);
        // println!("{:?}", t);
        s == t
    }
}