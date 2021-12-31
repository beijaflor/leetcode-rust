// https://leetcode.com/submissions/detail/602532183
impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut iterator = arrays.into_iter();
        let mut first_item = iterator.next().unwrap();
        let mut min = *first_item.iter().next().unwrap();
        let mut max = *first_item.iter().next_back().unwrap();
        let mut result = 0;
        
        iterator.for_each(|array| {
            let current_min = *array.iter().next().unwrap();
            let current_max = *array.iter().next_back().unwrap();
            let current_result = i32::max(max - current_min, current_max - min);
            min = i32::min(min, current_min);
            max = i32::max(max, current_max);
            result = i32::max(result, current_result);
        });
        
        result
    }
}