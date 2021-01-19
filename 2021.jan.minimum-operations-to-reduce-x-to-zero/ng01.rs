    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut steps = 2;
        let mut sums: Vec<i32> = vec![];
        let mut pointer_left = 0;
        let mut pointer_right = nums.len() - 1;
        sums.push(nums[pointer_left]);
        sums.push(nums[pointer_right]);
        pointer_left += 1;
        pointer_right -= 1;
        if nums[pointer_left] == x || nums[pointer_right] == x {
            return 1
        }
        
        while pointer_right > 0 {
            let left = nums[pointer_left];
            let right = nums[pointer_right];
            let orig = sums.clone();
            sums = vec![];
            println!("orig: {:?}", orig);
            for num in orig {
                println!("[BEFORE] left: {}, right: {}, num: {}", left, right, num);
                let left = left + num;
                let right = right + num;
                println!("[AFTER]  left: {}, right: {}", left, right);
                if left == x || right == x {
                    return steps
                }
                if right < x {
                    sums.push(right);
                }
                if left < x {
                    sums.push(left);
                }
            }
            if sums.len() < 1 {
                return -1
            }
            pointer_left += 1;
            pointer_right -= 1;
            steps += 1;
            println!("steps: {}, sums: {:?}", steps, sums);
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
}
