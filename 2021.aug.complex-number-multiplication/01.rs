// https://leetcode.com/submissions/detail/543364599/
impl Solution {
  pub fn complex_number_multiply(num1: String, num2: String) -> String {
      let x: Vec<i32> = num1[0..num1.len() - 1].split('+').map(|str| str.parse::<i32>().unwrap()).collect();
      let y: Vec<i32> = num2[0..num2.len() - 1].split('+').map(|str| str.parse::<i32>().unwrap()).collect();
      format!("{}+{}i", x[0] * y[0] - x[1] * y[1], x[0] * y[1] + x[1] * y[0])
  }
}