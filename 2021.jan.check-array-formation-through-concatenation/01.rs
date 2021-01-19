// https://leetcode.com/submissions/detail/440585786/
impl Solution {
  pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
      let mut clone_arr: Vec<Option<i32>> = arr.clone().into_iter().map(|x| Some(x)).collect();
      let max_len = clone_arr.len();
      for vec in pieces {
          if let Some(pos) = arr.iter().position(|x| x == &vec[0]) {
              if pos + vec.len() > max_len {
                  return false
              }
              let splice: Vec<_> = clone_arr.splice(pos..(pos + vec.len()), vec![None; vec.len()]).collect();
              if splice.contains(&None) {
                  return false
              }
              if splice.iter().map(|x| x.unwrap()).collect::<Vec<i32>>() != vec {
                  return false
              }
          } else {
              return false
          }
      }
      true
  }
}
