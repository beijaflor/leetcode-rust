fn calc(height: &Vec<i32>, left: usize, right: usize) -> i32 {
    i32::min(height[left], height[right]) * (right - left) as i32
}

    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left_pointer = 0;
        let mut right_pointer = height.len() - 1;
        let mut result = calc(&height, left_pointer, right_pointer);
        while left_pointer < right_pointer {
            let left_result = calc(&height, left_pointer + 1, right_pointer);
            let right_result = calc(&height, left_pointer, right_pointer - 1);
            println!("{}, {}", left_result, right_result);
            if left_result > right_result {
                result = i32::max(result, left_result);
                left_pointer += 1;
            } else {
                result = i32::max(result, right_result);
                right_pointer -= 1;
            }
            println!("{}", result);
        };
        result
    }



fn main() {
    assert_eq!(
        24,
        max_area(vec![1,3,2,5,25,24,5])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        49,
        max_area(vec![1,8,6,2,5,4,8,3,7])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        1,
        max_area(vec![1,1])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        16,
        max_area(vec![4,3,2,1,4])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        2,
        max_area(vec![1,2,1])
    );
    println!("SUCCESS\n\n");
}
