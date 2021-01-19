// https://leetcode.com/submissions/detail/439858741/
impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
      let mut len = 0;
      let mut buffer: Vec<char> = vec!();
      let chars: Vec<char> = s.chars().collect();
      for char in chars.iter() {
          if let Some(position) = buffer.iter().position(|x| x == char) {
              if len < buffer.len() {
                  len = buffer.len();
              }
              let (_, buf) = buffer.split_at(position + 1);
              buffer = buf.to_vec();
          };
          buffer.push(*char);
      }
      if buffer.len() > len {
          return buffer.len() as i32
      }
      return len as i32
  }
}
