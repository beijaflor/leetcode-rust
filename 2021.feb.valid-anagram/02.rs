// https://leetcode.com/submissions/detail/455039718/
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s = Vec::from(s);
        s.sort();
        let mut t = Vec::from(t);
        t.sort();
        // println!("{:?}", s);
        // println!("{:?}", t);
        s == t
    }
}