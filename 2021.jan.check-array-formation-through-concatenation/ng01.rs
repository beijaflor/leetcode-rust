impl Solution {
  pub fn can_form_array(mut arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
      let mut concat = pieces.into_iter().fold(vec!(), |mut acc, mut x| {
          acc.append(&mut x);
          acc
      });
      concat.sort();
      arr.sort();
      arr == concat
  }
}
