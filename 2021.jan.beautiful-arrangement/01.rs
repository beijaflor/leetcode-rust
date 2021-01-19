// https://leetcode.com/submissions/detail/438051527/
fn check(n: i32, index: i32) -> bool {
  n % index == 0 || index % n == 0
}

fn pos(vec: Vec<i32>, index: i32) -> Vec<i32> {
  vec.into_iter().filter(|x| check(*x, index)).collect()
}

fn dig(vec: Vec<i32>) -> i32 {
  let index = vec.len() as i32;
  if index == 1 {
      if check(vec[0], index) {
          return 1
      } else {
          return 0
      }
  }

  let orig_vec = vec.clone();
  let pos = pos(vec, index);
  // println!("pos: {:?}", pos);
  pos.into_iter().fold(0, |mut acc, num| {
      if check(num, index) {
          let new_vec = orig_vec.clone().into_iter().filter(|x| x != &num).collect();
          // println!("new_vec: {:?}", new_vec);
          acc += dig(new_vec);
      }
      acc
  })
}

impl Solution {
  pub fn count_arrangement(n: i32) -> i32 {
      let array: Vec<i32> = (1..n + 1).rev().collect();
      // println!("array: {:?}", array);
      dig(array)
  }
}
