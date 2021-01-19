// https://leetcode.com/submissions/detail/442810366/
impl Solution {
  pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
      people.sort();
      println!("{:?}", people);
      let mut pl: i32 = 0;
      let mut pr: i32 = people.len() as i32 - 1;
      let mut ship_count = 0;

      while pl <= pr {
          println!("{}, {}", pl, pr);
          ship_count += 1;

      
          if people[pl as usize] + people[pr as usize] <= limit {
              pl += 1;
          }
          pr -= 1;
      }

      ship_count
  }
}
