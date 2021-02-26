fn find_disorder(nums: &Vec<i32>, pos: usize) -> usize {
    let mut tmp = std::i32::MIN;
    for index in pos..nums.len() {
        if tmp > nums[index] {
            return index - 1
        } else {
            tmp = nums[index];
        }
    }
    return nums.len()
}

fn find_next(nums: &Vec<i32>, pos: usize, current: i32) -> usize {
    for index in pos..nums.len() {
        if current <= nums[index] {
            return index
        }
    }
    return nums.len()
}

    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let start = find_disorder(&nums, 0);
        if start == nums.len() { return 0 }
        let mut end = start;
        loop {
            let current = nums[end];
            let next_start = find_next(&nums, end + 1, current);
            if next_start == nums.len() {
                println!("start: {}, nums.len(): {}", start, nums.len());
                return (nums.len() - start) as i32
            }
            let next_end = find_disorder(&nums, next_start + 1);
            if next_end == nums.len() {
                println!("start: {}, next_start: {}", start, next_start);
                return (next_start - start) as i32
            }
            end = next_end;
            println!("start: {}, end: {}", start, end);
        }
    }



fn main() {
    assert_eq!(
        4,
        find_unsorted_subarray(vec![3,2,3,2,4])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        2,
        find_unsorted_subarray(vec![1,3,2,3,3])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        2,
        find_unsorted_subarray(vec![1,3,2,4,5])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        4,
        find_unsorted_subarray(vec![1,3,2,2,2])
    );
    println!("SUCCESS\n\n");
    
    assert_eq!(
        0,
        find_unsorted_subarray(vec![1,2,3,3,3])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        5,
        find_unsorted_subarray(vec![2,6,4,8,10,9,15])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        0,
        find_unsorted_subarray(vec![1,2,3,4])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        0,
        find_unsorted_subarray(vec![1])
    );
    println!("SUCCESS\n\n");

}
