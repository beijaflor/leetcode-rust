// https://leetcode.com/submissions/detail/443554861/
fn find_num(n: i32) -> i32 {
  if n < 2 { return n }
  let odd = n % 2;
  let modd = n / 2;
  let find;
  
  if odd == 1 {
      find = find_num(modd) + find_num(modd + 1);
  } else {
      find = find_num(modd);
  }
  // println!("num: {} => odd: {}, mod: {}, find: {}", n, odd, modd, find);
  find
}


impl Solution {
  pub fn get_maximum_generated(n: i32) -> i32 {
      let mut result = vec![];
      for index in 1..(n + 1) {
          result.push(find_num(index));
      }
      // println!("result: {:?}", result);
      result.sort();
      match result.last() {
        None => 0,
        Some(result) => *result,
    }
  }
}
