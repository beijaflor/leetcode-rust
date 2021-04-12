// https://leetcode.com/submissions/detail/479704974/
impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; n as usize];
        (0..n - k).for_each(|index| result[index as usize] = (index + 1) as i32);
        (0..k + 1).for_each(|index| {
            result[(index + n - k - 1) as usize] = if index % 2 == 0 { n - k + index / 2 } else { n - index / 2}
        });
        result
    }
}