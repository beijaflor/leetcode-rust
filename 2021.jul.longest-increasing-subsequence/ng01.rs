impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut iterator = nums.iter();
        let mut count = 1;
        if let Some(mut past) = iterator.next() {
            iterator.for_each(|num| {
                println!("{}, count: {}", num, count);
                if past < num {
                    count += 1;
                    result = i32::max(result, count);
                } else {
                    count = 1;
                }
                past = num;
            });
        } else {
            panic!("shouldn't reached to this line");
        }
        result
    }
}