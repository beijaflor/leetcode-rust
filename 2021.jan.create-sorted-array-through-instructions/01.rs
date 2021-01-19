// https://leetcode.com/submissions/detail/441443544/
struct BIT {
  size: i64,
  max: usize,
  counter: Vec<i32>,
}

impl BIT {
  fn init(max: usize) -> BIT {
      let mut counter = Vec::with_capacity(max + 1);
      for _ in 0..max + 1 {
          counter.push(0);
      }
      BIT { size: 0, max: max + 1, counter: counter }
  }
  
  fn increment(&mut self, num: i32) {
      let mut index = num;
      while index < self.max as i32 {
          // println!("inc index: {}", index);
          self.counter[index as usize] += 1;
          index += index & -index;
      }
      self.size += 1;
  }
  
  fn get_counts(&self, num: i32) -> i64 {
      let mut index = num;
      // println!("count: {}", index);
      let mut count: i64 = 0;

      while index > 0 {
          // println!("get index: {}", index);
          count += self.counter[index as usize] as i64;
          index -= index & -index;
      }
      count
  }
}

impl Solution {
  pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
      // let max = instructions.iter().fold(0, |m, v| *v.max(&m));
      // let mut bit = BIT::init(max as usize);
      let mut bit = BIT::init(100001);
      (instructions.iter().fold(0, |cost, num| {
          // println!("num: {}",  num);
          // println!("bit: {:?}",  bit.counter);
          let left = bit.get_counts(*num - 1);
          let right = bit.size - bit.get_counts(*num) as i64;
          bit.increment(*num);
          // println!("bit: {:?}",  bit.counter);
          // println!("left : {}\nright: {}\nsize : {}\n", left, right, bit.size);
          // println!("left: {}, right: {}, size: {}, num: {}, num_in_position: {}, bit: {:?}", left, right, bit.size, num,bit.counter[*num as usize], bit.counter);
          if left < right {
              return cost + left;
          } else {
              return cost + right;
          }
      }) % ( 1000000000 + 7 )) as i32
  }
}
