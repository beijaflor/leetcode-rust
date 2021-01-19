    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let max = nums.len();
        let mut steps = 1;

        println!("nums: {:?}, x: {}", nums, x);
        while steps <= max {
            println!("steps: {}", steps);
            for count in 0..(steps + 1) {
                let offset = max - steps + count;
                let left = (0..count).fold(0, |acc, index| acc + nums[index] );
                let right = (offset..max).fold(0, |acc, index| acc + nums[index] );
                println!("count: {}, left: {}, right: {}: sum: {}", count, left, right, left + right);
                if left + right == x {
                    return steps as i32
                }
            }
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
