// https://leetcode.com/submissions/detail/491829900/
impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let len = card_points.len() - 1;
        let mut total = (0..k).fold(0, |acc, index| acc + card_points[index]);
        let mut result = total;
        
        (0..k).for_each(|index| {
            total = total - card_points[k - index - 1] + card_points[len - index];
            result = i32::max(result, total);

            // println!("top: {}", card_points[k - index - 1]);
            // println!("last: {}", card_points[len - index]);
            // println!("total: {}", total);
            // println!("result: {}", result);
        });
        
        result
    }
}