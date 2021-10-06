/*
1
2
3
4
5
6
45
*/
impl Solution {
  pub fn climb_stairs(n: i32) -> i32 {
      if n < 3 { return n }
      Solution::climb_stairs(n - 1) + Solution::climb_stairs(n - 2)
  }
}