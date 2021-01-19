// https://leetcode.com/submissions/detail/440537998/
fn concat(parcials: Vec<String>) -> String {
  parcials.iter().fold("".to_string(), |acc, x| acc + x)
}

impl Solution {
  pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
      concat(word1) == concat(word2)
  }
}
