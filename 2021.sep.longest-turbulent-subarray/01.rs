// https://leetcode.com/submissions/detail/555676468/
impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        if arr.len() == 1 {
            1
        } else 
        if arr.len() == 2 {
            if arr[0] == arr[1] {
                1
            } else {
                2
            }
        }
        else {
            let mut flag = arr[0] > arr[1];
            let mut iterator = arr.into_iter();
            let mut last = iterator.next().unwrap();
            let mut count = 1;
            let mut result = 0;

            iterator.for_each(|num| {
                if (flag && last < num) || (!flag && last > num) {
                    count += 1;
                } else {
                    count = if last == num { 1 } else { 2 };
                }
                result = i32::max(result, count);
                flag = last > num;
                last = num;
            });
            
            result
        }
    }
}