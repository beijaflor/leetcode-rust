// https://leetcode.com/submissions/detail/484385415/
impl Solution {
    pub fn missing_number(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        let diff = (arr[len - 1] - arr[0]) / len as i32;
        if diff == 0 {
            return arr[0]
        }
        let mut last = arr[0];
        let position = (1..len).position(|index| {
            // println!("index: {}, v: {}, last: {}, diff: {}", index, arr[index], last, diff);
            if arr[index] - last == diff {
                last = arr[index];
                false
            } else {
                true
            }
        }).unwrap();
        // println!("{}", position);
        
        arr[position] + diff
    }
}