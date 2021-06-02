/*
"aabcc"
"dbbca"
"aadbbcbcac"
*/
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut p1 = 0;
        let mut p2 = 0;
        let mut bytes = s3.as_bytes();
        for index in 0..bytes.len() {
            println!("{}, {}, {}", index, p1, p2);
            if p1 < s1.len() && s1[p1] == bytes[index] {
                p1 += 1;
                continue
            } else if p2 < s2.len() && s2[p2] == bytes[index] {
                p2 += 1;
                continue
            }
            return false
        }
        true
    }
}