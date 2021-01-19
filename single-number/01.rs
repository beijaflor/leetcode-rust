impl Solution {
  pub fn single_number(nums: Vec<i32>) -> i32 {
      let mut n = nums.clone();
      n.sort();

      let mut iterator = n.into_iter();
      let mut current = iterator.next().unwrap();
      let mut flag = true;
      while let Some(i) = iterator.next() {
          if current == i {
              flag = false;
          } else {
              if flag {
                  return current
              } else {
                  current = i;
                  flag = true;
              }
          }
      }
      current
  }
}
