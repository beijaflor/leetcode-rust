// https://leetcode.com/submissions/detail/604229591/
impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();
        let (_, min) = arr.iter().fold((-10_000_000, i32::MAX), |(last, min), current| {
            (*current, i32::min(min, current - last))
        });

        let mut iterator = arr.into_iter();
        let mut last = iterator.next().unwrap();
        let mut result = Vec::<Vec<i32>>::new();
        iterator.for_each(|current| {
            // println!("{} {} {}", current, last, min);
            if current - last == min {
                result.push(vec![last, current]);
            }
            last = current;
        });
        
        result
    }
}