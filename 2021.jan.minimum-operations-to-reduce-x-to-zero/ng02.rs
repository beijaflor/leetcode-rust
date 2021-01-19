pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    let mut steps = 2;
    let mut pointer_left = 0;
    let mut pointer_right = nums.len() - 1;
    let mut left_tree: Vec<i32> = vec![nums[pointer_left]];
    let mut right_tree: Vec<i32> = vec![nums[pointer_right]];
  
    if x == nums[pointer_right] || x == nums[pointer_left] {
        return 1
    }
    
    while pointer_right > 0 {
        let orig = left_tree.clone();
        left_tree = vec![];
  
        println!("left_tree: {:?}", orig);
        let left = nums[pointer_left + 1];
        let right = nums[pointer_right];
        for num in &orig {
            println!("[BEFORE] left: {}, right: {}, num: {}", left, right, num);
            let left = left + num;
            let right = right + num;
            println!("[AFTER]  left: {}, right: {}", left, right);
            if left == x || right == x {
                return steps
            }
            if right < x {
                left_tree.push(right);
            }
            if left < x {
                left_tree.push(left);
            }
        }
  
        let left = nums[pointer_left];
        let right = nums[pointer_right - 1];
        for num in &orig {
            println!("[BEFORE] left: {}, right: {}, num: {}", left, right, num);
            let left = left + num;
            let right = right + num;
            println!("[AFTER]  left: {}, right: {}", left, right);
            if left == x || right == x {
                return steps
            }
            if right < x {
                left_tree.push(right);
            }
            if left < x {
                left_tree.push(left);
            }
        }
  
        let orig = right_tree.clone();
        right_tree = vec![];
  
        println!("right_tree: {:?}", orig);
  
        let left = nums[pointer_left];
        let right = nums[pointer_right - 1];
        for num in &orig {
            println!("[BEFORE] left: {}, right: {}, num: {}", left, right, num);
            let left = left + num;
            let right = right + num;
            println!("[AFTER]  left: {}, right: {}", left, right);
            if left == x || right == x {
                return steps
            }
            if right < x {
                right_tree.push(right);
            }
            if left < x {
                right_tree.push(left);
            }
        }
  
        let left = nums[pointer_left + 1];
        let right = nums[pointer_right];
        for num in &orig {
            println!("[BEFORE] left: {}, right: {}, num: {}", left, right, num);
            let left = left + num;
            let right = right + num;
            println!("[AFTER]  left: {}, right: {}", left, right);
            if left == x || right == x {
                return steps
            }
            if right < x {
                right_tree.push(right);
            }
            if left < x {
                right_tree.push(left);
            }
        }
  
        println!("steps: {}, left_tree: {:?}, left_tree: {:?}\n", steps, left_tree, right_tree);
  
        if left_tree.len() < 1 && right_tree.len() < 1 {
            return -1
        }
        pointer_left += 1;
        pointer_right -= 1;
        steps += 1;
    }
    -1
  }
  
  fn main() {
  assert_eq!(
    2,
    min_operations(vec![1,1,4,2,3], 5)
  );
  println!("SUCCESS\n\n");
  
  assert_eq!(
    -1,
    min_operations(vec![5,6,7,8,9], 4)
  );
  println!("SUCCESS\n\n");
  
  assert_eq!(
    5,
    min_operations(vec![3,2,20,1,1,3], 10)
  );
  println!("SUCCESS\n\n");
  
  assert_eq!(
    16,
    min_operations(vec![8828,9581,49,9818,9974,9869,9991,10000,10000,10000,9999,9993,9904,8819,1231,6309], 134365)
  );
  println!("SUCCESS\n\n");
  }
  