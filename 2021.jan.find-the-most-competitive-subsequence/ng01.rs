fn dig(nums: &Vec<i32>, max: usize) -> (Vec<i32>, i32) {
    let mut position = 0;
    let mut min = nums[0];
    for index in 1..nums.len() - max {
        if min > nums[index] {
            position = index;
            min = nums[index];
        }
    }
    (nums[position + 1..nums.len()].to_vec(), min)
}

    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(k as usize);
        let mut tmp_vec = nums.clone();
        for index in (0..(k as usize)).rev() {
            let (new_vec, num) = dig(&tmp_vec, index);
            result.push(num);
            tmp_vec = new_vec;
            println!("tmp_vec: {:?}, result: {:?}", tmp_vec, result);
        }
        result
    }

fn main() {
    assert_eq!(
        vec![2,6],
        most_competitive(vec![3,5,2,6], 2)
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        vec![2,3,3,4],
        most_competitive(vec![2,4,3,3,5,4,9,6], 4)
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        vec![2,3,3,1],
        most_competitive(vec![2,4,3,3,5,4,9,1], 4)
    );
    println!("SUCCESS\n\n");
}
