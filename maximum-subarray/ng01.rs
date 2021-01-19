impl Solution {
  pub fn max_sub_array(nums: Vec<i32>) -> i32 {

  let mut sub_vecs: Vec<Vec<i32>> = vec![];
  let slice = nums.as_slice();
  for i in 1..nums.len() + 1 {
      for j in 0..(nums.len() - i + 1) {
          let mut v: Vec<i32> = vec![];
          for k in j..(i + j) {
              v.push(*slice.get(k).unwrap());
          }
          sub_vecs.push(v);
      }
  }
  
  let mut iterator = sub_vecs.into_iter();
  let mut max: i32 = iterator.next().unwrap().into_iter().fold(0, |acc, x| acc + x);
  while let Some(vec) = iterator.next() {
      let sum = vec.into_iter().fold(0, |acc, x| acc + x);
      if sum > max {
          max = sum;
      }
  }
  max 
      
  }
}
