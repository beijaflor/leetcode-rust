// https://leetcode.com/submissions/detail/599145639/
fn dig(arr: &mut Vec<i32>, start: i32) -> bool {
    let jump = arr[start as usize];
    if jump == 0 { return true }
    if jump < 0 { return false }

    arr[start as usize] = -jump;
    if (start - jump) >= 0 && dig(arr, start - jump) {
        return true
    }
    if start + jump < arr.len() as i32 && dig(arr, start + jump) {
        return true
    }
    arr[start as usize] = jump;
    false
}

impl Solution {
    pub fn can_reach(mut arr: Vec<i32>, start: i32) -> bool {
        // if let Some(position_zero) = arr.iter().position(|x| *x == 0) {
        //     true
        // } else {
        //     panic!("no gaol")
        // }
        
        dig(&mut arr, start)
    }
}