pub fn find_lhs(nums: Vec<i32>) -> i32 {
  let mut result = 0;
  let mut count = 1;
  let mut interator = nums.iter();
  if let Some(start_num) = interator.next() {
      let mut start_num = *start_num;
      let mut sec_num: Option<i32> = None;
      interator.for_each(|v| {
          if let Some(sec) = sec_num {
              if *v == sec || *v == start_num {
                  count += 1;
              } else {
                  result = i32::max(result, count);
                  start_num = *v;
                  sec_num = None;
              }
          } else {
              if *v == start_num {
                  count += 1;
              } else if *v + 1 == start_num || *v - 1 == start_num {
                  sec_num = Some(*v);
                  count += 1;
              } else {
                  start_num = *v;
                  count = 1;
              }
          }
      });
      if sec_num != None {
          result = i32::max(result, count);
      }
  }
  result
}

fn main() {

assert_eq!(
  5,
  find_lhs(vec![1,3,2,2,5,2,3,7])
);
println!("SUCCESS\n\n");

assert_eq!(
  2,
  find_lhs(vec![1,2,3,4])
);
println!("SUCCESS\n\n");

assert_eq!(
  0,
  find_lhs(vec![1,1,1,1])
);
println!("SUCCESS\n\n");

}
