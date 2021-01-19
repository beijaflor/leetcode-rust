impl Solution {
  pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
      let mut new_array: Vec<i32> = vec![];
      instructions.iter().fold(0, |mut total_cost, num| {
          if let Some(position) = new_array.iter().position(|x| x >= num) {
              if let Some(back_position) = new_array.iter().position(|x| x > num) {
                  let right = new_array.len() - back_position;
                  if position < right {
                      total_cost += position as i32;
                  } else {
                      total_cost += right as i32;
                  }
              }
              new_array.insert(position, *num);
          } else {
              new_array.push(*num);
          }
          total_cost
      })
  }
}
