// https://leetcode.com/submissions/detail/537620982/
impl Solution {
  pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
      if s == words[0] { return true }

      let mut p = 0;
      let mut l = 0;
      while l < s.len() && p < words.len() {
          l += words[p].len();
          p += 1;
      }

      let s2 = words[0..p].into_iter().fold(String::from(""), |acc, str| { acc + str });
      // println!("{} {} {}", p, l, s2);
      s == s2
  }
}