fn devide(n: i32) -> Vec<i32> {
  let mut num = n;
  let mut result: Vec<i32> = vec![];
  while num > 0 {
      result.insert(0, num % 10);
      num /= 10;
  }
  result
}

impl Solution {
  pub fn is_happy(n: i32) -> bool {

  let mut num = n;
  let mut memo: Vec<i32> = vec![];

  while num != 1 {
      let calc: Vec<i32> = devide(num);
      num = calc.into_iter().fold(0, |acc, x| acc + x * x);
      if memo.contains(&num) {
          return false
      }
      memo.push(num);
  }
  return true

  }
}
