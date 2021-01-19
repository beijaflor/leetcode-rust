// https://leetcode.com/submissions/detail/445053996/
// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }

fn dig(nested_list: Vec<NestedInteger>, depth: i32) -> i32 {
  // println!("depth: {}, list: {:?}", depth, nested_list);
  let mut result = 0;
  for item in nested_list {
      match item {
          NestedInteger::Int(num) => result += num * depth,
          NestedInteger::List(list) => result += dig(list, depth + 1)
      }
  }
  result
}

impl Solution {
  pub fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
      dig(nested_list, 1)
  }
}