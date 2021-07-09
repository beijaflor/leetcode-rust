impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut sub_array: Vec<i32> = Vec::new();
        let mut result = 1;
        let mut iterator = nums.into_iter();
        sub_array.push(iterator.next().unwrap());
        iterator.for_each(|num| {
            println!("{:?}", sub_array);
            while let Some(last) = sub_array.last() {
                if last >= &num {
                    let index = sub_array.len();
                    if index > 1 {
                        if sub_array[index - 2] == num {
                            break
                        // } else {
                        //     sub_array.pop();                        
                        }
                    // } else {
                    //     sub_array.pop();
                    }
                    sub_array.pop();
                } else {
                    break
                }
            }
            if sub_array.last().unwrap_or(&0) < &num {
                sub_array.push(num);
                result = i32::max(result, sub_array.len() as i32);
            }
        });
        result
    }
}