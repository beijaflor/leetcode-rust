// https://leetcode.com/submissions/detail/484379689/
impl Solution {
    pub fn missing_number(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        let diff = (arr[len - 1] - arr[0]) / len as i32;
        let mut start = 0;
        let mut end = len - 1;
        // println!("diff: {}", diff);
        while start != end {
            // println!("start: {}/ end: {}", start, end);
            let middle = start + ((end - start) / 2);
            // println!("middle: {}", middle);
            // println!("{} / {}", arr[middle] - arr[start], (middle - start) as i32 * diff);
            if (arr[middle] - arr[start]) != (middle - start) as i32 * diff {
                end = middle;
            } else if (arr[end] - arr[middle + 1]) != (end - middle - 1) as i32 * diff {
                // println!("{} / {}", arr[end] - arr[middle + 1], (end - middle + 1) as i32 * diff);
                start = middle + 1;
            } else {
                start = middle + 1;
                end = middle + 1;
            }
        }
        // println!("{}, {}\n\n", start, end);
        arr[start] - diff
    }
}